use engine::Engine;
use log::{debug, error};
use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::constants::{WINDOW_DEFAULT_HEIGHT, WINDOW_DEFAULT_WIDTH};

pub mod constants;
pub mod engine;
pub mod utils;

fn main() {
    utils::logging::setup_logger(log::LevelFilter::Debug);

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = WindowBuilder::new()
        .with_title("Open Mojave")
        .with_inner_size(Size::Logical(LogicalSize::new(
            WINDOW_DEFAULT_WIDTH,
            WINDOW_DEFAULT_HEIGHT,
        )))
        .build(&event_loop)
        .expect("Failed to create window");

    let mut engine = Engine::new(window);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event }
                if window_id == engine.render_ctx.window.id() =>
            {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => {
                        // TODO: Window resized
                        engine.render_ctx.resize(size)
                    }

                    WindowEvent::RedrawRequested => {
                        engine.render_ctx.update();
                        match engine.render_ctx.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => {
                                engine.render_ctx.resize(engine.render_ctx.size)
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => error!("{:?}", e),
                        }
                        // TODO: render here
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                device_id: _,
                event: _,
            } => {}
            Event::AboutToWait => {
                // TODO: Lag updates
                // TODO: Redraw
                engine.render_ctx.window.request_redraw();
            }

            _ => {}
        })
        .unwrap();
}
