# Plugin Core

Plugin Core 是一個 Rust 外掛系統核心庫，提供了簡單且靈活的外掛開發框架。

## 功能特性

- 簡單的外掛 trait 定義
- 提供便捷的過程宏用於外掛開發
- 動態載入/卸載外掛
- 類型安全的外掛介面

## 組件說明

該專案包含以下主要組件：

- `plugin_api`: 定義了外掛系統的核心 trait 和類型
- `plugin_macro`: 提供用於簡化外掛開發的過程宏

## 使用方法

### 1. 使用腳手架建立基礎框架
```shell=
chmmod-create -n <模組名稱>
```
詳細見[chmmod-create](https://github.com/End-YYDS/chmmod-create)
### 2. 範例Plugin
```rust
use plugin_core::plugin_api::Plugin;
use plugin_core::plugin_macro::{plugin_entry, plugin_exit};
#[plugin_entry]
#[plugin_exit]
#[derive(Debug)]
struct Example;
const PLUGIN_NAME: &str = "Example";
const PLUGIN_VERSION: &str = "0.1.0";
const PLUGIN_DESCRIPTION: &str = "This is Example Plugin";
impl Plugin for Example {
    fn name(&self) -> &str {
        PLUGIN_NAME 
    }
    fn version(&self) -> &str {
        PLUGIN_VERSION
    }
    fn description(&self) -> &str {
        PLUGIN_DESCRIPTION
    }
    fn execute(&self, input: Option<&str>) -> PluginResult<()> {
        // 實現外掛邏輯
        let command: PluginCommand = serde_json::from_str(input).map_err(|e| {
            PluginError::command_error(
                input,
                format!("Failed to parse command: {}", e),
            )
        })?;

        // 處理不同的命令
        match command.action.as_str() {
            // 看需要執行什麼
        }
        Ok(())
    }
    fn load() -> xBox<dyn Plugin> {
        Box::new(Example)
    }
    fn unload(){
    // 實現外掛卸載邏輯
    }
}
```
## API 文檔

### Plugin Trait

外掛必須實現 `Plugin` trait，包含以下方法：

- `name()`: 返回外掛名稱
- `version()`: 返回外掛版本
- `description()`: 返回外掛描述
- `execute()`: 執行外掛功能
- `load()`: 載入外掛
- `unload()`: 卸載外掛

### 過程宏

- `#[plugin_entry]`: 生成外掛入口點
- `#[plugin_exit]`: 生成外掛卸載點

### 插件Error Type
我們提供了六種錯誤類型來處理不同情況下的錯誤：
- `LoadError`
用於處理外掛加載過程中發生的錯誤：
`context`: 錯誤發生的上下文信息  
`message`: 具體的錯誤信息

- `ExecutionError`
用於處理外掛執行過程中的錯誤：
`context`: 錯誤發生的上下文信息  
`message`: 具體的錯誤信息

- `CommandError`
用於處理命令解析或處理過程中的錯誤：  
`command`: 導致錯誤的命令  
`message`: 具體的錯誤信息

- `ConfigurationError`
用於處理配置相關的錯誤：
`key`: 問題配置項  
`message`: 具體的錯誤信息

- `ResourceError`
用於處理資源訪問或處理錯誤：
`resource_type`: 資源類型  
`message`: 具體的錯誤信息

- `CustomError`
用於處理特定插件的特殊錯誤情況：
`error_type`: 錯誤類型  
`message`: 具體的錯誤信息

所有這些錯誤類型都實現了`std::error::Error`和 `std::fmt::Display` traits，提供了標準的錯誤處理能力和格式化輸出。