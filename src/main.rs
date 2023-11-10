use bevy_ecs::{prelude::*, system::RunSystemOnce};
use constants::VERSION;
use engine::{
    events::{KeyboardEvent, MouseEvent, WindowResizeEvent},
    renderer::RenderContext,
    systems::schedules::{BeforeUpdate, Render, Startup, Update},
};
use log::{debug, error};
use winit::{
    dpi::{LogicalSize, Size},
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::constants::{WINDOW_DEFAULT_HEIGHT, WINDOW_DEFAULT_WIDTH};

pub mod constants;
pub mod engine;
pub mod utils;

fn main() {
    utils::logging::setup_logger(log::LevelFilter::Debug);

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // Create the window
    let window = WindowBuilder::new()
        .with_title(format!("Open Mojave v{}", VERSION))
        .with_inner_size(Size::Logical(LogicalSize::new(
            WINDOW_DEFAULT_WIDTH,
            WINDOW_DEFAULT_HEIGHT,
        )))
        .build(&event_loop)
        .expect("Failed to create window");

    // Create the render context from the window
    let render_context: RenderContext = pollster::block_on(RenderContext::new(&window));

    let mut world = World::default();

    // Add render context as a resource
    world.insert_resource(render_context);

    // Run the startup schedule
    {
        let mut schedule = Schedule::new(Startup);
        schedule.add_systems((
            // Initialize
            engine::systems::init::init,
        ));
        schedule.run(&mut world);
    }

    // Create before update schedule
    {
        let mut schedule = Schedule::new(BeforeUpdate);
        // schedule.add_systems((
        // ));

        world.add_schedule(schedule);
    }

    // Create update schedule
    {
        let mut schedule = Schedule::new(Update);
        // schedule.add_systems((
        // ));

        world.add_schedule(schedule);
    }

    // Create render schedule
    {
        let mut schedule = Schedule::new(Render);
        // schedule.add_systems((
        // ));

        world.add_schedule(schedule);
    }

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { window_id, event } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::Resized(new_size) => {
                            world.send_event(WindowResizeEvent { new_size })
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            world.send_event(MouseEvent::Button {
                                button,
                                pressed: state == ElementState::Pressed,
                            })
                        }
                        WindowEvent::MouseWheel { delta, phase, .. } => {
                            world.send_event(MouseEvent::Scroll { delta, phase })
                        }

                        WindowEvent::KeyboardInput { event, .. } => {
                            world.send_event(KeyboardEvent {
                                key: event.logical_key,
                                pressed: event.state == ElementState::Pressed,
                            })
                        }
                        WindowEvent::RedrawRequested => {
                            world.run_schedule(BeforeUpdate);
                            world.run_schedule(Update);
                            world.run_schedule(Render);
                        }
                        _ => {}
                    }
                }
                // Mouse moved
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => world.send_event(MouseEvent::Move { delta }),
                Event::AboutToWait => {
                    // TODO: Lag updates

                    window.request_redraw();
                }

                _ => {}
            }
        })
        .unwrap();
}
