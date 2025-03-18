use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDescriptor {
    pub name: String,
    pub data_type: String,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub id: String,
    pub version: String,
    pub description: String,
    pub inputs: Vec<PortDescriptor>,
    pub outputs: Vec<PortDescriptor>,
    #[serde(default)]
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingData {
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
    pub parameters: serde_json::Value,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Plugin loading error: {0}")]
    Loading(String),
    #[error("Execution error: {0}")]
    Execution(String),
    #[error("Script Execution error: {0}")]
    ScriptExecution(#[from] rustyscript::Error),
    #[error("Dynamic library panic: {0}")]
    DynamicPanic(String),
}
