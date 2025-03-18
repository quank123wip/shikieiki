
// src/loader.rs
use crate::node::{rust_plugin::RustPlugin, ts_plugin::TsPlugin, plugin::Plugin};
use std::{path::Path, sync::Arc};

pub enum PluginType {
    Rust(String),
    TypeScript(String),
}

pub struct PluginLoader;

impl PluginLoader {
    pub unsafe fn load(self: &Self, plugin: PluginType) -> Box<dyn Plugin> {
        match plugin {
            PluginType::Rust(path) => {
                let plugin = RustPlugin::new(&path).unwrap();
                Box::new(plugin)
            }
            PluginType::TypeScript(path) => {
                let plugin = TsPlugin::new(&path).unwrap();
                Box::new(plugin)
            }
        }
    }
}
