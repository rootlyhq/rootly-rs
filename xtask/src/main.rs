use std::fs;
use std::path::Path;

/// Remove enums with operator values like `!=`, `>=`, `<=`, `=`
/// that can't become valid Rust identifiers.
fn sanitize_enums(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Array(variants)) = map.get("enum") {
                let has_operator = variants
                    .iter()
                    .any(|v| matches!(v.as_str(), Some("!=" | ">=" | "<=" | "=")));
                if has_operator {
                    map.remove("enum");
                    return;
                }
            }
            for v in map.values_mut() {
                sanitize_enums(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                sanitize_enums(v);
            }
        }
        _ => {}
    }
}

/// Strip 4xx/5xx/default error responses — Progenitor panics on multiple
/// response types. We handle errors in the hand-written client wrapper.
fn strip_error_responses(spec: &mut serde_json::Value) {
    let paths = match spec.get_mut("paths") {
        Some(serde_json::Value::Object(p)) => p,
        _ => return,
    };

    for (_path, methods) in paths.iter_mut() {
        let methods = match methods.as_object_mut() {
            Some(m) => m,
            None => continue,
        };

        for method in &["get", "post", "put", "patch", "delete"] {
            let op = match methods.get_mut(*method) {
                Some(serde_json::Value::Object(o)) => o,
                _ => continue,
            };

            let responses = match op.get_mut("responses") {
                Some(serde_json::Value::Object(r)) => r,
                _ => continue,
            };

            let error_codes: Vec<String> = responses
                .keys()
                .filter(|code| code.starts_with('4') || code.starts_with('5') || *code == "default")
                .cloned()
                .collect();

            for code in error_codes {
                responses.remove(&code);
            }
        }
    }
}

/// Strip `default` values from nullable fields — typify can't render defaults
/// for Option types (e.g. `default: false` on `nullable: true` boolean).
fn strip_nullable_defaults(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if map.get("nullable") == Some(&serde_json::Value::Bool(true)) {
                map.remove("default");
            }
            for v in map.values_mut() {
                strip_nullable_defaults(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                strip_nullable_defaults(v);
            }
        }
        _ => {}
    }
}

/// Simplify path parameter schemas that use `anyOf` (e.g. UUID-or-slug)
/// to plain `type: string`. Progenitor generates complex types for `anyOf`
/// which triggers an assertion failure in Builder mode.
fn simplify_path_params(spec: &mut serde_json::Value) {
    let paths = match spec.get_mut("paths") {
        Some(serde_json::Value::Object(p)) => p,
        _ => return,
    };

    for (_path, methods) in paths.iter_mut() {
        let methods = match methods.as_object_mut() {
            Some(m) => m,
            None => continue,
        };

        // Path-level parameters
        if let Some(serde_json::Value::Array(params)) = methods.get_mut("parameters") {
            for param in params.iter_mut() {
                simplify_if_path_param(param);
            }
        }

        // Operation-level parameters
        for method in &["get", "post", "put", "patch", "delete"] {
            let op = match methods.get_mut(*method) {
                Some(serde_json::Value::Object(o)) => o,
                _ => continue,
            };

            if let Some(serde_json::Value::Array(params)) = op.get_mut("parameters") {
                for param in params.iter_mut() {
                    simplify_if_path_param(param);
                }
            }
        }
    }
}

fn simplify_if_path_param(param: &mut serde_json::Value) {
    let map = match param.as_object_mut() {
        Some(m) => m,
        None => return,
    };

    if map.get("in") != Some(&serde_json::Value::String("path".to_string())) {
        return;
    }

    if let Some(schema) = map.get_mut("schema") {
        if schema.get("anyOf").is_some() || schema.get("oneOf").is_some() {
            *schema = serde_json::json!({"type": "string"});
        }
    }
}

fn main() {
    let spec_path = Path::new("spec/swagger.json");
    if !spec_path.exists() {
        eprintln!("Error: spec/swagger.json not found. Run `make fetch-spec` first.");
        std::process::exit(1);
    }

    let spec_str = fs::read_to_string(spec_path).expect("failed to read spec file");
    let mut spec_value: serde_json::Value =
        serde_json::from_str(&spec_str).expect("failed to parse spec as JSON");

    sanitize_enums(&mut spec_value);
    strip_error_responses(&mut spec_value);
    strip_nullable_defaults(&mut spec_value);
    simplify_path_params(&mut spec_value);

    let spec: openapiv3::OpenAPI =
        serde_json::from_value(spec_value).expect("failed to parse spec as OpenAPI");

    let mut settings = progenitor::GenerationSettings::default();
    settings.with_interface(progenitor::InterfaceStyle::Builder);
    settings.with_tag(progenitor::TagStyle::Merged);

    let mut generator = progenitor::Generator::new(&settings);

    let tokens = generator
        .generate_tokens(&spec)
        .expect("failed to generate tokens from spec");

    let ast = syn::parse2(tokens).expect("failed to parse generated tokens");
    let code = prettyplease::unparse(&ast);

    let output_path = Path::new("src/generated.rs");
    fs::write(output_path, &code).expect("failed to write generated.rs");

    println!(
        "Generated {} bytes to {}",
        code.len(),
        output_path.display()
    );
}
