mod proto_gen {
    tonic::include_proto!("lock");
}

use proto_gen::lock_service_client::LockServiceClient;
use proto_gen::AcquireLockRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LockServiceClient::connect("http://[::1]:50051").await?;
    let resp = client.acquire_lock(AcquireLockRequest {
        lock_id: "demo".into(),
        owner_id: "cli".into(),
        lease_ttl_ms: 5000,
    }).await?;
    println!("Acquire: {:?}", resp.into_inner());
    Ok(())
}
