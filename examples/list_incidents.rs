use rootly::RootlyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token =
        std::env::var("ROOTLY_API_TOKEN").expect("ROOTLY_API_TOKEN environment variable required");

    let rootly = RootlyClient::from_token(token);

    let response = rootly.client().list_incidents().page_size(5).send().await?;

    let incidents = response.into_inner();
    for item in &incidents.data {
        println!("- [{}] {}", item.id, item.attributes.title);
    }

    Ok(())
}
