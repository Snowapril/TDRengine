use crate::{device_error::DeviceError, vulkan::physical_device::PhysicalDevice};
use ash::vk::{
    KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn, KhrPortabilitySubsetFn,
};
use ash::{extensions::ext, vk, Entry};
use std::ffi::{c_char, CStr};

pub struct InstanceDescription {
    extension_names: Vec<&'static CStr>,
    layer_names: Vec<&'static CStr>,
    app_name: &'static CStr,
    engine_name: &'static CStr,
}

pub struct Instance {
    entry: Entry,
    instance: ash::Instance,
}

impl Instance {
    pub fn create(instance_desc: InstanceDescription) -> Result<Instance, DeviceError> {
        let entry = unsafe { Entry::load() }.map_err(|err| DeviceError::from(err.to_string()))?;

        // let layer_names = [CStr::from_bytes_with_nul_unchecked(
        //     b"VK_LAYER_KHRONOS_validation\0",
        // )];
        let layers_names_raw: Vec<*const c_char> = instance_desc
            .layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        #[allow(unused_mut)]
        let mut extension_names_raw: Vec<*const c_char> = instance_desc
            .extension_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            extension_names_raw.push(KhrPortabilityEnumerationFn::name().as_ptr());
            // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
            extension_names_raw.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        }

        // TODO(snowapril) : support to control app, engine, api version management
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(instance_desc.app_name)
            .application_version(0)
            .engine_name(instance_desc.engine_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0))
            .build();

        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::default()
        };

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names_raw)
            .flags(create_flags)
            .build();

        let instance =
            unsafe { entry.create_instance(&create_info, None) }.map_err(DeviceError::from)?;

        Ok(Self { entry, instance })
    }

    pub fn get_desire_physical_device<F>(
        &self,
        desired_queue_flags: vk::QueueFlags,
        optional_condition: Option<F>,
    ) -> Result<(vk::PhysicalDevice, u32), DeviceError>
    where
        F: Fn(&vk::PhysicalDevice, u32) -> bool,
    {
        let physical_devices_available =
            unsafe { self.instance.enumerate_physical_devices() }.map_err(DeviceError::from)?;
        // let surface_loader =
        //     Surface::new(device_desc.instance.entry(), device_desc.instance.get_handle());
        Ok(physical_devices_available
            .iter()
            .find_map(|pdevice| {
                unsafe {
                    self.instance
                        .get_physical_device_queue_family_properties(*pdevice)
                }
                .iter()
                .enumerate()
                .find_map(|(index, info)| {
                    let supports_flags_and_extra_condition =
                        info.queue_flags.contains(desired_queue_flags)
                            && optional_condition
                                .as_ref()
                                .map(|f| f(pdevice, index as u32))
                                .unwrap_or(true);
                    if supports_flags_and_extra_condition {
                        Some((*pdevice, index as u32))
                    } else {
                        None
                    }
                })
            })
            .expect("Couldn't find suitable device."))
    }

    #[inline]
    pub fn get_handle(&self) -> &ash::Instance {
        &self.instance
    }

    #[inline]
    pub fn entry(&self) -> &Entry {
        &self.entry
    }
}
