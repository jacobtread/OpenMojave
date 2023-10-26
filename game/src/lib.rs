//! Game project.
use config::GameConfiguration;
use font::set_default_font;
use fyrox::{
    core::log::Log,
    core::{algebra::Vector2, pool::Handle, sstorage::ImmutableString},
    engine::GraphicsContext,
    event::Event,
    event_loop::ControlFlow,
    gui::{
        button::ButtonBuilder, message::UiMessage, ttf::Font, utils, widget::WidgetBuilder,
        UserInterface,
    },
    material::{Material, PropertyValue},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    resource::texture::{TextureResource, TextureResourceExtension},
    scene::{ Scene},
    utils::translate_event,
};

mod bsa;
mod config;
mod font;

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
}

impl Game {
    pub fn new(override_scene: Option<&str>, mut context: PluginContext) -> Self {
        let config = config::load_config();

        set_default_font(&mut context.user_interface);

        // Create some widgets as usual.
        ButtonBuilder::new(WidgetBuilder::new())
            .with_text("Click Me!")
            .build(&mut context.user_interface.build_ctx());
        

        Self { config }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }
    
fn update(&mut self,  context: &mut PluginContext) {
            // Add your global update code here.

}

    fn on_os_event(
        &mut self,
        event: &Event<()>,
        context: PluginContext,
    ) {

        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &UiMessage,
    ) {

        // Handle UI events here.
    }
}
