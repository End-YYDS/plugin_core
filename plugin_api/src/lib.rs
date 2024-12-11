pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, input: &str) -> String;
    fn unload()
    where
        Self: Sized;
    fn load() -> Box<dyn Plugin>
    where
        Self: Sized;
}

// 定义插件的工厂函数类型
pub type CreatePluginFn = fn() -> Box<dyn Plugin>;
pub type UnloadPluginFn = fn();
