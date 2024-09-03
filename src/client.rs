use tonic::transport::Channel;
use crate::info::info_service_client::InfoServiceClient;
use crate::info::InfoRequest;

pub mod info {
    tonic::include_proto!("info");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("URL").unwrap_or("http://localhost:9999"
        .to_string());

    let interval_string = std::env::var("INTERVAL").unwrap_or("3".to_string());
    let interval = interval_string.parse::<u64>()?;

    let mut client = InfoServiceClient::connect(url).await?;

    loop {
        let hostname = get_hostname(&mut client).await?;

        println!("hostname {:?}", hostname);

        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
    }
}

async fn get_hostname(client: &mut InfoServiceClient<Channel>) -> Result<String, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(InfoRequest {});

    let response = client.info(request).await?;

    Ok(response.into_inner().hostname)
}