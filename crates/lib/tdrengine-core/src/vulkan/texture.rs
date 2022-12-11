use crate::{
    device_error::DeviceError, vulkan::device::Device, vulkan::instance::Instance,
    vulkan::physical_device::PhysicalDevice, window::Window,
};
use ash::{extensions::khr, vk};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::ffi::CStr;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

pub struct Texture {
    image: vk::Image,
    image_memory: vk::DeviceMemory,
    memory_type_index: u32,
    alloc_info: vk::MemoryAllocateInfo,
}

impl Texture {
    pub fn create(device: &Device, create_info: vk::ImageCreateInfo) -> Result<Self, DeviceError> {
        let device_memory_properties = unsafe {
            device
                .get_instance()
                .get_handle()
                .get_physical_device_memory_properties(*device.get_physical_device().get_handle())
        };

        let image = unsafe { device.get_handle().create_image(&create_info, None) }
            .map_err(DeviceError::from)?;
        let image_memory_req = unsafe { device.get_handle().get_image_memory_requirements(image) };
        let memory_type_index = find_memorytype_index(
            &image_memory_req,
            &device_memory_properties,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
        .ok_or_else(|| {
            DeviceError::from(String::from(
                "Unable to find suitable memory index for depth image.",
            ))
        })?;

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(image_memory_req.size)
            .memory_type_index(memory_type_index)
            .build();

        let image_memory = unsafe { device.get_handle().allocate_memory(&alloc_info, None) }
            .map_err(DeviceError::from)?;

        unsafe {
            device
                .get_handle()
                .bind_image_memory(image, image_memory, 0)
        }
        .map_err(DeviceError::from)?;

        Ok(Self {
            image,
            image_memory,
            memory_type_index,
            alloc_info,
        })
    }
}

// TODO(snowapril) : convert u32 to proper enum types for memory type
fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}
