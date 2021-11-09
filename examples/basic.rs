use containerd;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

const ENDPOINT: &str = "http://192.168.1.143:15432";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ns = std::env::var("CONTAINERD_NS").map_err(|_| {
        "Pass a valid containerd namesapce via the `CONTAINERD_NS` environment variable."
            .to_string()
    })?;

    let header_value = MetadataValue::from_str(&ns)?;

    let channel = Channel::from_static(ENDPOINT).connect().await?;
    let mut service =
        containerd::api::services::leases::v1::leases_client::LeasesClient::with_interceptor(
            channel,
            move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("containerd-namespace", header_value.clone());
                Ok(req)
            },
        );

    let res = service
        .list(Request::new(
            containerd::api::services::leases::v1::ListRequest { filters: vec![] },
        ))
        .await?;

    println!("RESPONSE={:?}", res.get_ref().leases);
    Ok(())
}
