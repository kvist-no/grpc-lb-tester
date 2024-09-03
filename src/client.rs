use tonic::transport::Channel;
use crate::info::info_service_client::InfoServiceClient;
use crate::info::InfoRequest;
use crate::status::status_service_client::StatusServiceClient;
use crate::status::StatusRequest;

pub mod info {
    tonic::include_proto!("info");
}

pub mod status {
    tonic::include_proto!("status");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("URL").unwrap_or("http://localhost:9999"
        .to_string());

    let interval_string = std::env::var("INTERVAL").unwrap_or("3".to_string());
    let interval = interval_string.parse::<u64>()?;

    let mut info_client = InfoServiceClient::connect(url.clone()).await?;
    let mut status_client = StatusServiceClient::connect(url.clone()).await?;

    loop {
        let (elapsed_info, result_info) = time_async_call(get_hostname(&mut
            info_client))
            .await;

        let (elapsed_status, _) = time_async_call(invoke_status_code
            (&mut
            status_client))
            .await;

        let hostname = result_info?;

        println!("Hostname : {:?} ({}ms)", hostname, elapsed_info);
        println!("Status   : {}ms", elapsed_status);

        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
    }
}

async fn get_hostname(client: &mut InfoServiceClient<Channel>) -> Result<String, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(InfoRequest {});

    let response = client.info(request).await?;

    Ok(response.into_inner().hostname)
}

async fn invoke_status_code(client: &mut StatusServiceClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(StatusRequest {
        code: "Unavailable".to_string()
    });

    client.status(request).await?;

    Ok(())
}

async fn time_async_call<F, T>(
    f: F
) -> (u128, Result<T, Box<dyn std::error::Error>>)
where
    F: std::future::Future<Output=Result<T, Box<dyn std::error::Error>>>,
{
    let now = std::time::Instant::now();
    let result = f.await;
    let elapsed = now.elapsed().as_millis();

    (elapsed, result)
}