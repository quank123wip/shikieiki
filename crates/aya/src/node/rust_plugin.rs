use crate::node::manager::NodeManager;
use crate::node::models::{NodeMetadata, PluginError, ProcessingData};
use async_trait::async_trait;
use libloading::{Library, Symbol};

use crate::node::plugin::Plugin;

pub struct RustPlugin {
    // lib: Library,
    metadata: NodeMetadata,
    process_fn: unsafe fn(ProcessingData) -> Result<ProcessingData, PluginError>,
}

impl RustPlugin {
    pub fn new(path: &str) -> Result<Self, PluginError> {
        unsafe {
            let lib = Library::new(path).unwrap();

            let metadata_fn: Symbol<fn() -> NodeMetadata> =
                lib.get::<fn() -> NodeMetadata>(b"metadata").unwrap();
            let process_fn: Symbol<
                unsafe fn(ProcessingData) -> Result<ProcessingData, PluginError>,
            > = lib.get(b"process").unwrap();

            let metadata = metadata_fn();

            // Move `lib` and `Symbol`s into the struct
            Ok(Self {
                // lib,
                metadata,
                process_fn: *process_fn,
            })
        }
    }
}

#[async_trait]
impl Plugin for RustPlugin {
    fn metadata(&self) -> &NodeMetadata {
        &self.metadata
    }
async fn process(&mut self, data: ProcessingData, _manager: &NodeManager) -> Result<ProcessingData, PluginError> {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let data = AssertUnwindSafe(data);
    let result = catch_unwind(|| {
    unsafe { (self.process_fn)(data.0) }
});

    match result {
        Ok(inner_result) => inner_result,
        Err(e) => {
            let error_msg = format!("Dynamic library panic: {:?}", e);
            return Err(PluginError::DynamicPanic(error_msg));
        }
    }
}
}
