# rootly-rs

Strongly-typed Rust SDK for the [Rootly](https://rootly.com) API, generated from the OpenAPI spec using [Progenitor](https://github.com/oxidecomputer/progenitor).

- **Full coverage** — all 415+ API operations with builder pattern
- **Strongly typed** — every endpoint, parameter, and response is a Rust type
- **Async/await** — powered by [reqwest](https://github.com/seanmonstar/reqwest) and [tokio](https://tokio.rs)
- **JSON:API compliant** — handles `application/vnd.api+json` content negotiation

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rootly = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use rootly::RootlyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RootlyClient::from_token(std::env::var("ROOTLY_API_TOKEN")?);

    let response = client.client().list_incidents().page_size(10).send().await?;

    for item in &response.data {
        println!("{}: {}", item.id, item.attributes.title);
    }

    Ok(())
}
```

## Examples

### List Incidents

```rust
let response = client
    .client()
    .list_incidents()
    .filter_status("started".to_string())
    .page_number(1)
    .page_size(10)
    .include("causes,services".to_string())
    .send()
    .await?;

for item in &response.data {
    println!(
        "[{}] {} ({})",
        item.id,
        item.attributes.title,
        item.attributes.status.as_deref().unwrap_or("unknown"),
    );
}
```

### Get an Incident

```rust
let response = client
    .client()
    .get_incident()
    .id("abc123")
    .include("sub_statuses,causes,subscribers".to_string())
    .send()
    .await?;

let incident = &response.data.attributes;
println!("Title: {}", incident.title);
println!("Summary: {}", incident.summary.as_deref().unwrap_or(""));
```

### Create an Incident

```rust
use rootly::types::{NewIncident, NewIncidentData, NewIncidentDataAttributes, NewIncidentDataType};

let body = NewIncident::builder()
    .data(
        NewIncidentData::builder()
            .type_(NewIncidentDataType::Incidents)
            .attributes(
                NewIncidentDataAttributes::builder()
                    .title("Service degradation".to_string())
                    .summary("Users experiencing elevated latency".to_string())
                    .severity_id("sev-1".to_string()),
            ),
    )
    .build()
    .unwrap();

let response = client.client().create_incident().body(body).send().await?;
```

### Update an Incident

```rust
use rootly::types::{UpdateIncident, UpdateIncidentData, UpdateIncidentDataAttributes, UpdateIncidentDataType};

let body = UpdateIncident::builder()
    .data(
        UpdateIncidentData::builder()
            .type_(UpdateIncidentDataType::Incidents)
            .attributes(
                UpdateIncidentDataAttributes::builder()
                    .summary("Root cause identified — deploying fix".to_string()),
            ),
    )
    .build()
    .unwrap();

let response = client
    .client()
    .update_incident()
    .id("abc123")
    .body(body)
    .send()
    .await?;
```

### List Services

```rust
let response = client.client().list_services().page_size(50).send().await?;

for svc in &response.data {
    println!("{}: {}", svc.id, svc.attributes.name);
}
```

### Create an Alert

```rust
use rootly::types::{NewAlert, NewAlertData, NewAlertDataAttributes, NewAlertDataType};

let body = NewAlert::builder()
    .data(
        NewAlertData::builder()
            .type_(NewAlertDataType::Alerts)
            .attributes(
                NewAlertDataAttributes::builder()
                    .summary("High error rate on payments service".to_string())
                    .source("datadog".to_string()),
            ),
    )
    .build()
    .unwrap();

let response = client
    .client()
    .create_alert()
    .alerts_source_id("src-123")
    .body(body)
    .send()
    .await?;
```

### Pagination

```rust
let mut page = 1;
loop {
    let response = client
        .client()
        .list_incidents()
        .page_number(page)
        .page_size(25)
        .send()
        .await?;

    let data = response.into_inner();
    if data.data.is_empty() {
        break;
    }

    for item in &data.data {
        println!("{}: {}", item.id, item.attributes.title);
    }

    page += 1;
}
```

### Error Handling

```rust
use rootly::ClientError;

match client.client().get_incident().id("nonexistent").send().await {
    Ok(response) => println!("Found: {}", response.data.attributes.title),
    Err(ClientError::ErrorResponse(resp)) => {
        eprintln!("API error {}: {}", resp.status(), resp.status());
    }
    Err(e) => eprintln!("Request failed: {e}"),
}
```

### Retry with Backoff

For handling rate limits (HTTP 429):

```rust
use rootly::retry::{with_backoff, RateLimitConfig};

let config = RateLimitConfig {
    max_retries: 5,
    initial_backoff_ms: 500,
};

let response = with_backoff(config, || {
    client.client().list_incidents().page_size(100).send()
}).await?;
```

## Configuration

```rust
use rootly::{RootlyClient, RootlyClientConfig};

// Custom base URL
let client = RootlyClient::new(RootlyClientConfig {
    token: "your-token".into(),
    base_url: "https://custom.rootly.com".into(),
});

// Quick setup with defaults
let client = RootlyClient::from_token("your-token");
```

## Using Types

All generated types are available under `rootly::types`:

```rust
use rootly::types::{
    Incident,
    IncidentList,
    NewIncident,
    Service,
    Alert,
    Severity,
    Team,
    // ... 685 total types
};
```

## API Coverage

Full coverage of the Rootly API v1 (415+ operations):

| Resource | Operations |
|----------|-----------|
| Incidents | list, get, create, update, delete |
| Services | list, get, create, update, delete |
| Alerts | list, get, create, update, resolve, snooze, escalate |
| Teams | list, get, create, update, delete |
| Severities | list, get, create, update, delete |
| Environments | list, get, create, update, delete |
| Workflows | list, get, create, update, delete |
| Playbooks | list, get, create, update, delete |
| Schedules | list, get, create, update, delete |
| Escalation Policies | list, get, create, update, delete |
| Dashboards | list, get, create, update, delete |
| Status Pages | list, get, create, update, delete |
| Custom Fields | list, get, create, update, delete |
| Catalogs | list, get, create, update, delete |
| ... and more | |

## Development

```bash
make regenerate   # Full clean + fetch spec + generate + build
make fetch-spec   # Download latest OpenAPI spec from Rootly
make generate     # Run Progenitor code generation
make build        # Compile
make test         # Run tests
make clippy       # Lint
make fmt          # Format
make clean        # Remove build artifacts + generated code
```

### How Generation Works

The `xtask` crate handles code generation with four spec sanitization passes:

1. **Sanitize enums** — strips operator enums (`=`, `!=`, `>=`, `<=`) that can't be Rust identifiers
2. **Strip error responses** — removes 4xx/5xx responses (Progenitor limitation)
3. **Strip nullable defaults** — removes defaults from nullable fields (typify limitation)
4. **Simplify path params** — flattens `anyOf` path params to plain strings

Then [Progenitor](https://github.com/oxidecomputer/progenitor) generates a fully typed async client with builder-pattern methods from the preprocessed OpenAPI spec.

## Requirements

- Rust 1.85+
- tokio runtime

## License

MIT — see [LICENSE](LICENSE)
