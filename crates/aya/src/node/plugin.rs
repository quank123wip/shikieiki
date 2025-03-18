use async_trait::async_trait;

use crate::node::models::{NodeMetadata, ProcessingData};
use crate::node::manager::NodeManager;

use super::models::PluginError;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &NodeMetadata;
    async fn process(&mut self, data: ProcessingData, _manager: &NodeManager) -> Result<ProcessingData, PluginError>;
}