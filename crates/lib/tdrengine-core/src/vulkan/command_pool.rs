use ash::vk;

use crate::{
    device_error::DeviceError,
    vulkan::{command_buffer::CommandBuffer, device::Device, physical_device::PhysicalDevice},
};
use std::{cell::RefCell, rc::Rc};

pub struct CommandPool {
    handle: vk::CommandPool,
    freed_command_buffers: RefCell<Vec<CommandBuffer>>,
    device: Rc<Device>,
}

impl CommandPool {
    pub fn create(
        physical_device: &PhysicalDevice,
        device: Rc<Device>,
    ) -> Result<Self, DeviceError> {
        let pool_create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(physical_device.queue_family_index());

        Ok(Self {
            handle: unsafe {
                device
                    .get_handle()
                    .create_command_pool(&pool_create_info, None)
            }
            .map_err(DeviceError::from)?,
            freed_command_buffers: RefCell::new(Vec::new()),
            device,
        })
    }

    pub fn get_command_buffer(&self) -> Option<CommandBuffer> {
        // get command_buffer
        if self.freed_command_buffers.borrow().is_empty() {
            // allocate new one or make error
            None
        } else {
            self.freed_command_buffers.borrow_mut().pop()
        }
    }

    pub fn release_command_buffer(&self, cmd_buffer: CommandBuffer) {}

    pub fn preallocate_command_buffers(&self, num_cmd_buffers: u32) -> Result<(), DeviceError> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(num_cmd_buffers)
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY);

        self.freed_command_buffers.borrow_mut().extend(
            unsafe {
                self.device
                    .get_handle()
                    .allocate_command_buffers(&command_buffer_allocate_info)
            }
            .map_err(DeviceError::from)?
            .into_iter()
            .map(|vk_command_buffer| CommandBuffer::create(vk_command_buffer))
            .collect::<Vec<CommandBuffer>>(),
        );

        Ok(())
    }
}
