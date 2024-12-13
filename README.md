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
    fn execute(&self, input: &str) -> String {
        // 實現外掛邏輯
        format!("處理輸入: {}", input)
    }
    fn load() -> Box<dyn Plugin> {
        Box::new(Example)
    }
    fn unload() {
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