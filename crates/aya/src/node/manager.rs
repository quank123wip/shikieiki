use rustyscript::{worker::{DefaultWorker, DefaultWorkerOptions, Worker}, Runtime, RuntimeOptions};
use tokio::sync::Mutex;
use std::{sync::Arc, time::Duration};

pub struct NodeManager {
    pub worker: Arc<Mutex<DefaultWorker>>,
}

impl NodeManager {
    pub fn new() -> Self {
        let worker = DefaultWorker::new(DefaultWorkerOptions { 
            timeout: Duration::from_millis(5000), 
            default_entrypoint: Some("run".to_string()), 
            ..Default::default()
        }).expect("Failed to create worker");
        NodeManager { worker: Arc::new(Mutex::new(worker)) }
    }
}
