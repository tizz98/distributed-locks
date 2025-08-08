mod api;

mod proto_gen {
    tonic::include_proto!("lock");
}

use api::LockServiceImpl;
use tonic::transport::Server;
use proto_gen::lock_service_server::LockServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc = LockServiceImpl::default();

    println!("Lock server listening on {}", addr);

    Server::builder()
        .add_service(LockServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
