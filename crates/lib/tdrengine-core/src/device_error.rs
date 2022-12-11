use std::{backtrace::Backtrace, fmt};

#[derive(Debug)]
pub enum DeviceError {
    Vulkan {
        err: ash::vk::Result,
        trace: Backtrace,
    },
    TDRError {
        log: String,
        trace: Backtrace,
    },
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DeviceError::Vulkan { err, trace } => {
                write!(f, "Vulkan error: {err:?}; {trace:?}")
            }
            DeviceError::TDRError { log, trace } => {
                write!(f, "TDR internal error : {log:?}, {trace:?}")
            }
        }
    }
}

impl std::error::Error for DeviceError {}

impl From<ash::vk::Result> for DeviceError {
    fn from(err: ash::vk::Result) -> Self {
        Self::Vulkan {
            err,
            trace: Backtrace::capture(),
        }
    }
}

impl From<String> for DeviceError {
    fn from(log: String) -> Self {
        Self::TDRError {
            log,
            trace: Backtrace::capture(),
        }
    }
}
