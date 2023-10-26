//! Executor with your game connected to it as a plugin.
use fyrox::{
    dpi::LogicalSize,
    engine::{executor::Executor, GraphicsContextParams},
    event_loop::EventLoop,
    window::WindowAttributes,
};
use open_mojave::GameConstructor;

fn main() {
    let mut window_attributes = WindowAttributes::default();
    window_attributes.resizable = true;
    window_attributes.title = "Open Mojave".to_string();
    window_attributes.inner_size = Some(fyrox::dpi::Size::Logical(LogicalSize {
        width: 960.,
        height: 540.,
    }));

    let mut executor = Executor::from_params(
        EventLoop::new().unwrap(),
        GraphicsContextParams {
            window_attributes,
            vsync: true,
        },
    );

    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}
