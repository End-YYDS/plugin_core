use std::error::Error as StdError;
use std::fmt;

/// 定義插件系統中可能發生的所有錯誤類型
#[derive(Debug)]
#[allow(dead_code)]
pub enum PluginError {
    /// 插件加載過程中發生的錯誤
    LoadError {
        /// 錯誤發生的上下文信息
        context: String,
        /// 具體的錯誤信息
        message: String,
    },
    /// 插件執行過程中發生的錯誤
    ExecutionError {
        /// 錯誤發生的上下文信息
        context: String,
        /// 具體的錯誤信息
        message: String,
    },
    /// 命令解析或處理過程中的錯誤
    CommandError {
        /// 導致錯誤的命令
        command: String,
        /// 具體的錯誤信息
        message: String,
    },
    /// 配置相關的錯誤
    ConfigurationError {
        /// 問題配置項
        key: String,
        /// 具體的錯誤信息
        message: String,
    },
    /// 資源訪問或處理錯誤
    ResourceError {
        /// 資源類型
        resource_type: String,
        /// 具體的錯誤信息
        message: String,
    },
    /// 自定義錯誤，用於處理特定插件的特殊錯誤情況
    CustomError {
        /// 錯誤類型
        error_type: String,
        /// 具體的錯誤信息
        message: String,
    },
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginError::LoadError { context, message } => {
                write!(f, "Load error in {}: {}", context, message)
            }
            PluginError::ExecutionError { context, message } => {
                write!(f, "Execution error in {}: {}", context, message)
            }
            PluginError::CommandError { command, message } => {
                write!(f, "Command '{}' error: {}", command, message)
            }
            PluginError::ConfigurationError { key, message } => {
                write!(f, "Configuration error for '{}': {}", key, message)
            }
            PluginError::ResourceError {
                resource_type,
                message,
            } => {
                write!(f, "{} resource error: {}", resource_type, message)
            }
            PluginError::CustomError {
                error_type,
                message,
            } => {
                write!(f, "{}: {}", error_type, message)
            }
        }
    }
}

impl StdError for PluginError {}

#[allow(dead_code)]
/// 提供便捷的錯誤創建方法
impl PluginError {
    pub fn load_error<S: Into<String>>(context: S, message: S) -> Self {
        Self::LoadError {
            context: context.into(),
            message: message.into(),
        }
    }

    pub fn execution_error<S: Into<String>>(context: S, message: S) -> Self {
        Self::ExecutionError {
            context: context.into(),
            message: message.into(),
        }
    }

    pub fn command_error<S: Into<String>>(command: S, message: S) -> Self {
        Self::CommandError {
            command: command.into(),
            message: message.into(),
        }
    }

    pub fn configuration_error<S: Into<String>>(key: S, message: S) -> Self {
        Self::ConfigurationError {
            key: key.into(),
            message: message.into(),
        }
    }

    pub fn resource_error<S: Into<String>>(resource_type: S, message: S) -> Self {
        Self::ResourceError {
            resource_type: resource_type.into(),
            message: message.into(),
        }
    }

    pub fn custom_error<S: Into<String>>(error_type: S, message: S) -> Self {
        Self::CustomError {
            error_type: error_type.into(),
            message: message.into(),
        }
    }
}

#[allow(dead_code)]
/// 為插件操作定義結果類型別名
pub type PluginResult<T> = Result<T, PluginError>;
