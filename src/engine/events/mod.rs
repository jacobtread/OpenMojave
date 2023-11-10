use bevy_ecs::event::Event;
use winit::{
    dpi::PhysicalSize,
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::Key,
};

/// Event from a keyboard key being pressed
#[derive(Debug, Event)]
pub struct KeyboardEvent {
    /// The key code for this event
    pub key: Key,
    /// Whether the key is pressed
    pub pressed: bool,
}

#[derive(Debug, Event)]
pub enum MouseEvent {
    /// Mouse was moved
    Move {
        /// Mouse delta
        delta: (f64, f64),
    },

    /// Mouse scrolled
    Scroll {
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },

    /// Mouse button was pressed
    Button {
        /// The mouse button this event was for
        button: MouseButton,
        /// Whether the button is presed
        pressed: bool,
    },
}

/// Event for the window size changing
#[derive(Debug, Event)]
pub struct WindowResizeEvent {
    /// The new size of the window
    pub new_size: PhysicalSize<u32>,
}
