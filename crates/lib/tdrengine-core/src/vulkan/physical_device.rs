use crate::{
    device_error::DeviceError, vulkan::device::Device, vulkan::instance::Instance,
    vulkan::swapchain::Surface, window::Window,
};
use ash::vk;
pub struct PhysicalDevice {
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
}

impl PhysicalDevice {
    pub fn create(instance: &Instance, surface: &Surface) -> Result<Self, DeviceError> {
        let (surface_handle, surface_loader) = (surface.get_handle(), surface.loader());
        let (physical_device, queue_family_index) = instance.get_desire_physical_device(
            vk::QueueFlags::GRAPHICS,
            Some(|pdevice: &vk::PhysicalDevice, index| {
                unsafe {
                    surface_loader.get_physical_device_surface_support(
                        *pdevice,
                        index,
                        *surface_handle,
                    )
                }
                .unwrap()
            }),
        )?;

        Ok(Self {
            physical_device,
            queue_family_index,
        })
    }

    pub fn get_handle(&self) -> &vk::PhysicalDevice {
        &self.physical_device
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
}
