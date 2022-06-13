use std::sync::Arc;

use egui_winit_vulkano::Gui;
use vulkano::device::{Device, Queue};

use crate::renderer::swapchain::SwapchainFrame;

pub mod scene;
pub mod window;

pub struct GuiState<'a> {
    pub gui: &'a mut Gui,

    pub frame: &'a SwapchainFrame<'a>,
}

pub struct GuiRenderer<'a> {
    pub gui: &'a mut Gui,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub format: vulkano::format::Format,
}