use tdrengine_core::{Window, Instance, Device};

pub struct TestDescription {
    width: u32,
    height: ui32,
    title: &'static str,
}

pub trait RenderingTest {
    pub fn create(test_desc: TestDescription) -> RenderTest;

    pub fn update();
    pub fn render();

    pub fn render_loop() {
        loop {
            update();
            render();
        };
    }
}