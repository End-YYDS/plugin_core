mod error;
use error::PluginResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

pub trait Plugin: Debug {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, input: &str) -> PluginResult<()>;
    fn unload() 
    where
        Self: Sized;
    fn load() -> Box<dyn Plugin>
    where
        Self: Sized;
}
pub type CreatePluginFn = fn() -> Box<dyn Plugin>;
pub type UnloadPluginFn = fn() ;
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginCommand {
    pub action: String,
    pub parameters: Value,
}
// #[derive(serde::Deserialize, Debug)]
// pub struct ModuleConfig {
//     pub name: String,
//     pub version: String,
//     pub description: String,
// }

// impl ModuleConfig {
//     pub fn load(path: &str) -> Self {
//         let content = std::fs::read_to_string(path).expect("Failed to read Mod.toml");
//         let config: toml::Value = toml::from_str(&content).expect("Failed to parse Mod.toml");

//         config
//             .get("module")
//             .expect("Missing [module] section in Mod.toml")
//             .clone()
//             .try_into()
//             .expect("Failed to deserialize [module] section")
//     }
// }
