use crate::{
    device_error::DeviceError,
    vulkan::{command_pool::CommandPool, device::Device, physical_device::PhysicalDevice},
};
use ash::vk;
use std::{marker::Copy, rc::Rc};

pub struct CommandBuffer {
    handle: vk::CommandBuffer,
}

impl CommandBuffer {
    pub fn create(handle: vk::CommandBuffer) -> Self {
        Self { handle }
    }
}
