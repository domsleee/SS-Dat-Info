use log::info;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct CancellationRegistry {
    requests: Arc<Mutex<HashMap<Uuid, Arc<AtomicBool>>>>,
}

impl CancellationRegistry {
    pub fn create_and_register_task(&self) -> (Uuid, Arc<AtomicBool>) {
        let task_id = Uuid::new_v4();
        let token = Arc::new(AtomicBool::new(false));
        self.requests.lock().unwrap().insert(task_id, token.clone());

        (task_id, token)
    }

    pub fn cancel_task(&self, id: &Uuid) -> bool {
        let registry = self.requests.lock().unwrap();
        if let Some(token) = registry.get(id) {
            info!("Cancelling download with ID: {id}");
            token.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn remove_task(&self, id: &Uuid) -> bool {
        let mut registry = self.requests.lock().unwrap();
        registry.remove(id).is_some()
    }

    pub fn instance() -> &'static CancellationRegistry {
        &REGISTRY
    }
}

pub fn is_task_cancelled(token: &Arc<AtomicBool>) -> bool {
    token.load(Ordering::SeqCst)
}

static REGISTRY: Lazy<CancellationRegistry> = Lazy::new(CancellationRegistry::default);
