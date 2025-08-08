use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct LockState {
    pub owner_id: String,
    pub expiry: Instant,
    pub fencing_token: u64,
}

#[derive(Default, Debug)]
pub struct LockManager {
    locks: HashMap<String, LockState>,
    token_counter: u64,
}

impl LockManager {
    pub fn acquire_lock(
        &mut self,
        lock_id: &str,
        owner_id: &str,
        lease_ttl: Duration,
    ) -> (bool, u64, String) {
        let now = Instant::now();
        match self.locks.get(lock_id) {
            Some(lock) if lock.expiry > now && lock.owner_id != owner_id => {
                (false, 0, "Lock already held".into())
            }
            _ => {
                self.token_counter += 1;
                self.locks.insert(
                    lock_id.to_string(),
                    LockState {
                        owner_id: owner_id.into(),
                        expiry: now + lease_ttl,
                        fencing_token: self.token_counter,
                    },
                );
                (true, self.token_counter, "Lock acquired".into())
            }
        }
    }

    pub fn release_lock(&mut self, lock_id: &str, owner_id: &str) -> (bool, String) {
        match self.locks.get(lock_id) {
            Some(lock) if lock.owner_id == owner_id => {
                self.locks.remove(lock_id);
                (true, "Lock released".into())
            }
            _ => (false, "Lock not owned or does not exist".into()),
        }
    }

    pub fn renew_lock(
        &mut self,
        lock_id: &str,
        owner_id: &str,
        lease_ttl: Duration,
    ) -> (bool, String) {
        let now = Instant::now();
        match self.locks.get_mut(lock_id) {
            Some(lock) if lock.owner_id == owner_id && lock.expiry > now => {
                lock.expiry = now + lease_ttl;
                (true, "Lock renewed".into())
            }
            _ => (false, "Lock not owned or expired".into()),
        }
    }

    pub fn get_status(
        &self,
        lock_id: &str,
    ) -> (bool, String, u64, Duration) {
        let now = Instant::now();
        match self.locks.get(lock_id) {
            Some(lock) if lock.expiry > now => (
                true,
                lock.owner_id.clone(),
                lock.fencing_token,
                lock.expiry.saturating_duration_since(now),
            ),
            _ => (false, "".into(), 0, Duration::from_secs(0)),
        }
    }

    pub fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.locks.retain(|_, v| v.expiry > now);
    }
}
