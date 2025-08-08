use std::{sync::{Arc, Mutex}, time::Duration};
use tonic::{Request, Response, Status};
use crate::proto_gen::*;
use lock_core::LockManager;

#[derive(Default)]
pub struct LockServiceImpl {
    manager: Arc<Mutex<LockManager>>,
}

#[tonic::async_trait]
impl lock_service_server::LockService for LockServiceImpl {
    async fn acquire_lock(
        &self,
        request: Request<AcquireLockRequest>,
    ) -> Result<Response<AcquireLockResponse>, Status> {
        let req = request.into_inner();
        let mut mgr = self.manager.lock().unwrap();
        let (acquired, token, message) =
            mgr.acquire_lock(&req.lock_id, &req.owner_id, Duration::from_millis(req.lease_ttl_ms as u64));
        Ok(Response::new(AcquireLockResponse {
            acquired,
            fencing_token: token as i64,
            message,
        }))
    }

    async fn release_lock(
        &self,
        request: Request<ReleaseLockRequest>,
    ) -> Result<Response<ReleaseLockResponse>, Status> {
        let req = request.into_inner();
        let mut mgr = self.manager.lock().unwrap();
        let (released, message) = mgr.release_lock(&req.lock_id, &req.owner_id);
        Ok(Response::new(ReleaseLockResponse { released, message }))
    }

    async fn renew_lock(
        &self,
        request: Request<RenewLockRequest>,
    ) -> Result<Response<RenewLockResponse>, Status> {
        let req = request.into_inner();
        let mut mgr = self.manager.lock().unwrap();
        let (renewed, message) =
            mgr.renew_lock(&req.lock_id, &req.owner_id, Duration::from_millis(req.lease_ttl_ms as u64));
        Ok(Response::new(RenewLockResponse { renewed, message }))
    }

    async fn get_lock_status(
        &self,
        request: Request<GetLockStatusRequest>,
    ) -> Result<Response<GetLockStatusResponse>, Status> {
        let req = request.into_inner();
        let mgr = self.manager.lock().unwrap();
        let (is_locked, owner_id, fencing_token, lease_duration) = mgr.get_status(&req.lock_id);
        Ok(Response::new(GetLockStatusResponse {
            is_locked,
            owner_id,
            fencing_token: fencing_token as i64,
            lease_expiry_ms: lease_duration.as_millis() as i64,
        }))
    }
}
