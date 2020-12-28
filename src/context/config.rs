use crate::CanvasConfig;
use crate::DisplayConfig;
use crate::Vector;
use crate::{graphics::TextureConfig, quicksilver_compat::Color};
use web_sys::HtmlCanvasElement;

#[derive(Default)]
pub struct PaddleConfig {
    pub display: DisplayConfig,
    pub enable_text_board: bool,
}

impl PaddleConfig {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_canvas_id(mut self, id: &'static str) -> Self {
        self.display.canvas = CanvasConfig::HtmlId(id);
        self
    }
    pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.display.canvas = CanvasConfig::HtmlElement(canvas);
        self
    }
    pub fn with_resolution(mut self, pixels: impl Into<Vector>) -> Self {
        self.display.pixels = pixels.into();
        self
    }
    pub fn with_texture_config(mut self, texture_config: TextureConfig) -> Self {
        self.display.texture_config = texture_config;
        self
    }
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.display.background = Some(color);
        self
    }
    pub fn with_text_board(mut self) -> Self {
        self.enable_text_board = true;
        self
    }
    pub fn without_text_board(mut self) -> Self {
        self.enable_text_board = false;
        self
    }
}
