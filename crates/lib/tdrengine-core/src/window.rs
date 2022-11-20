use crate::device_error::DeviceError;
use glam;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::cell::RefCell;
use std::ffi::CStr;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

pub struct WindowDescription {
    width: u32,
    height: u32,
    title: &'static str,
    event_loop: EventLoop<()>,
}

pub struct Window {
    handle: winit::window::Window,
    event_loop: RefCell<EventLoop<()>>,
    width: u32,
    height: u32,
}

impl Window {
    pub fn create(window_desc: WindowDescription) -> Result<Self, DeviceError> {
        let event_loop = window_desc.event_loop;
        let window = WindowBuilder::new()
            .with_title(window_desc.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                f64::from(window_desc.width),
                f64::from(window_desc.height),
            ))
            .build(&event_loop)
            .map_err(|err| DeviceError::from(err.to_string()))?;

        Ok(Self {
            handle: window,
            event_loop: RefCell::new(event_loop),
            width: window_desc.width,
            height: window_desc.height,
        })
    }

    #[inline]
    pub fn get_handle(&self) -> &winit::window::Window {
        &self.handle
    }

    #[inline]
    pub fn extent(&self) -> glam::UVec2 {
        glam::UVec2 {
            x: self.width,
            y: self.height,
        }
    }

    #[inline]
    pub fn get_event_loop(&self) -> &RefCell<EventLoop<()>> {
        &self.event_loop
    }
}
