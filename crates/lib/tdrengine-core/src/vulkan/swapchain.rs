use crate::{
    device_error::DeviceError, vulkan::device::Device, vulkan::instance::Instance,
    vulkan::physical_device::PhysicalDevice, vulkan::texture::Texture, window::Window,
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

pub struct Surface {
    handle: vk::SurfaceKHR,
    surface_loader: khr::Surface,
}

impl Surface {
    pub fn create(instance: &Instance, window: &Window) -> Result<Self, DeviceError> {
        let surface = unsafe {
            ash_window::create_surface(
                instance.entry(),
                instance.get_handle(),
                window.get_handle().raw_display_handle(),
                window.get_handle().raw_window_handle(),
                None,
            )
            .map_err(DeviceError::from)?
        };
        let surface_loader = khr::Surface::new(instance.entry(), instance.get_handle());

        Ok(Self {
            handle: surface,
            surface_loader,
        })
    }

    #[inline]
    pub fn get_handle(&self) -> &vk::SurfaceKHR {
        &self.handle
    }

    #[inline]
    pub fn loader(&self) -> &khr::Surface {
        &self.surface_loader
    }
}

pub struct SwapChain {
    surface: Surface,
    handle: vk::SwapchainKHR,
    swapchain_loader: khr::Swapchain,
    present_images: Vec<vk::Image>,
    present_image_views: Vec<vk::ImageView>,
}

impl SwapChain {
    pub fn create(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        device: &Device,
        window: &Window,
    ) -> Result<Self, DeviceError> {
        let surface = Surface::create(instance, window)?;

        let surface_format = unsafe {
            surface.loader().get_physical_device_surface_formats(
                *physical_device.get_handle(),
                *surface.get_handle(),
            )
        }
        .unwrap()[0];

        let surface_capabilities = unsafe {
            surface.loader().get_physical_device_surface_capabilities(
                *physical_device.get_handle(),
                *surface.get_handle(),
            )
        }
        .unwrap();
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }
        let surface_resolution = match surface_capabilities.current_extent.width {
            std::u32::MAX => vk::Extent2D {
                width: window.extent().x,
                height: window.extent().y,
            },
            _ => surface_capabilities.current_extent,
        };
        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };
        let present_modes = unsafe {
            surface.loader().get_physical_device_surface_present_modes(
                *physical_device.get_handle(),
                *surface.get_handle(),
            )
        }
        .unwrap();
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);
        let swapchain_loader = khr::Swapchain::new(instance.get_handle(), device.get_handle());

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*surface.get_handle())
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain =
            unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None) }.unwrap();

        let present_images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }
            .map_err(DeviceError::from)?;
        let present_image_views: Result<Vec<vk::ImageView>, DeviceError> = present_images
            .iter()
            .map(|&image| {
                let create_view_info = vk::ImageViewCreateInfo::builder()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image);
                unsafe {
                    device
                        .get_handle()
                        .create_image_view(&create_view_info, None)
                }
                .map_err(DeviceError::from)
            })
            .collect();

        // let depth_image_create_info = vk::ImageCreateInfo::builder()
        //     .image_type(vk::ImageType::TYPE_2D)
        //     .format(vk::Format::D16_UNORM)
        //     .extent(surface_resolution.into())
        //     .mip_levels(1)
        //     .array_layers(1)
        //     .samples(vk::SampleCountFlags::TYPE_1)
        //     .tiling(vk::ImageTiling::OPTIMAL)
        //     .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        //     .sharing_mode(vk::SharingMode::EXCLUSIVE);
        //
        // let depth_image = Texture::create(device, depth_image_create_info)?;

        Ok(Self {
            surface,
            handle: swapchain,
            swapchain_loader,
            present_images,
            present_image_views: present_image_views?,
        })
    }
}
