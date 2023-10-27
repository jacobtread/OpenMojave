//! Game project.
use std::sync::Arc;

use bsa::BsaResourceIo;
use config::GameConfiguration;
use font::set_default_font;
use fyrox::{
    core::log::Log,
    core::{
        algebra::Vector2, futures::executor::block_on, pool::Handle, sstorage::ImmutableString,
    },
    engine::GraphicsContext,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    gui::{
        button::ButtonBuilder, message::UiMessage, ttf::Font, utils, widget::WidgetBuilder,
        UserInterface,
    },
    material::{Material, PropertyValue},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    resource::texture::{TextureResource, TextureResourceExtension},
    scene::Scene,
    utils::translate_event,
};
use menu::Menu;

mod bsa;
mod config;
mod font;
mod menu;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
    }

    fn create_instance(
        &self,
        override_scene: Option<&str>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(override_scene, context))
    }
}

pub struct Game {
    config: GameConfiguration,
    menu: Menu,
}

impl Game {
    pub fn new(override_scene: Option<&str>, mut context: PluginContext) -> Self {
        let config = config::load_config();

        // Update resource loading to use the bsa resource loader
        {
            // Collect the archives that must be loaded
            let archives: Vec<&str> = config.archive.SArchiveList.split(", ").collect();
            let io = BsaResourceIo::load(&archives, context.resource_manager.resource_io());

            let mut state = context.resource_manager.state();
            state.set_resource_io(Arc::new(io));
        }

        set_default_font(context.user_interface);

        let menu = block_on(Menu::new(&mut context));

        Self { config, menu }
    }

    pub fn on_window_resized(&mut self, ui: &UserInterface, width: f32, height: f32) {
        self.menu.resize(ui, width, height);
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) {
        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {}
                WindowEvent::Resized(new_size) => self.on_window_resized(
                    context.user_interface,
                    new_size.width as f32,
                    new_size.height as f32,
                ),
                _ => {}
            }
        }
        // Do something on OS event here.
    }

    fn on_ui_message(&mut self, context: &mut PluginContext, message: &UiMessage) {

        // Handle UI events here.
    }
}
