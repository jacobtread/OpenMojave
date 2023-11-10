use bevy_ecs::schedule::ScheduleLabel;

/// Startup schedule label
#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Startup;

/// BeforeUpdate schedule label
#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BeforeUpdate;

/// Update schedule label
#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Update;

/// Render schedule label
#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Render;
