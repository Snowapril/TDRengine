pub use crate::vulkan::{instance, swapchain};
pub use crate::{device_error::DeviceError, vulkan::physical_device::PhysicalDevice};
use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};
use ash::{vk, Entry};
pub use std::cell::RefCell;
pub use std::ffi::{c_char, CStr};
pub use std::rc::Rc;

pub struct DeviceDescription {
    extension_names: Vec<&'static CStr>,
    instance: Rc<instance::Instance>,
    surface: Rc<swapchain::Surface>,
    physical_device: Rc<PhysicalDevice>,
}

pub struct Device {
    handle: ash::Device,
    instance: Rc<instance::Instance>,
    physical_device: Rc<PhysicalDevice>,
}

impl Device {
    pub fn create(device_desc: DeviceDescription) -> Result<Self, DeviceError> {
        let mut device_extension_names_raw: Vec<*const c_char> = device_desc
            .extension_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        device_extension_names_raw.push(
            Swapchain::name().as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            KhrPortabilitySubsetFn::name().as_ptr(),
        );

        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(device_desc.physical_device.queue_family_index())
            .queue_priorities(&priorities)
            .build();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features)
            .build();

        Ok(Self {
            handle: unsafe {
                device_desc.instance.get_handle().create_device(
                    *device_desc.physical_device.get_handle(),
                    &device_create_info,
                    None,
                )
            }
            .map_err(DeviceError::from)?,
            instance: device_desc.instance,
            physical_device: device_desc.physical_device,
        })
    }

    #[inline]
    pub fn get_handle(&self) -> &ash::Device {
        &self.handle
    }

    #[inline]
    pub fn get_instance(&self) -> &Rc<instance::Instance> {
        &self.instance
    }

    #[inline]
    pub fn get_physical_device(&self) -> &Rc<PhysicalDevice> {
        &self.physical_device
    }
}
