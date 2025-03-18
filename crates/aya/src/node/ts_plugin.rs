use std::path::Path;
use std::result;

use async_trait::async_trait;
use rustyscript::js_value::Promise;
use serde_json::json;

use crate::node::manager::NodeManager;
use crate::node::models::{NodeMetadata, PluginError, ProcessingData};
use crate::node::plugin::Plugin;
use rustyscript::{Module, json_args};

pub struct TsPlugin {
    metadata: NodeMetadata,
    module: Module,
}

impl TsPlugin {
    pub fn new(path: &str) -> Result<Self, PluginError> {
        let module = std::fs::read_to_string(format!("{}/module.ts", path))
            .expect("Failed to read metadata file");
        let module = Module::new("module.ts", module);

        let metadata = std::fs::read_to_string(format!("{}/metadata.json", path))
            .expect("Failed to read metadata file");
        let metadata =
            serde_json::from_str::<NodeMetadata>(&metadata).expect("Failed to parse NodeMetadata");
        Ok(Self { metadata, module })
    }
}

#[async_trait]
impl Plugin for TsPlugin {
    fn metadata(&self) -> &NodeMetadata {
        &self.metadata
    }

    async fn process(
        &mut self,
        data: ProcessingData,
        _manager: &NodeManager,
    ) -> Result<ProcessingData, PluginError> {
        let context = json!(data.context).to_string();
        let payload = json!(data.payload).to_string();

        let worker_guard = _manager.worker.lock().await;
        let _exec: i32 = worker_guard.eval(format!(
            "
        const context = {};
        const payload = {};
        1
    ",
            context, payload
        ))?;
        let handle = worker_guard.load_module(self.module.clone())?;

        let result: ProcessingData = worker_guard.call_function(
            Some(handle),
            String::from("process"),
            vec![data.parameters],
        )?;
        Ok(result)
    }
}
