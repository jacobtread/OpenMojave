use winit::window::Window;

use self::renderer::RenderContext;

pub mod renderer;

pub struct Engine {
    pub render_ctx: RenderContext,
}

impl Engine {
    pub fn new(window: Window) -> Self {
        let render_ctx = pollster::block_on(RenderContext::new(window));

        Self { render_ctx }
    }
}
