//! Game project.
use bsa::BsaResourceIo;
use config::GameConfiguration;
use font::{load_font, set_default_font};
use fyrox::{
    core::futures::executor::block_on,
    event::{Event, WindowEvent},
    gui::{message::UiMessage, UserInterface},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
};
use menu::Menu;
use std::{fs::File, io::Cursor, path::Path, sync::Arc};

mod bsa;
mod config;
mod esm;
mod font;
mod menu;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
    }

    fn create_instance(
        &self,
        _override_scene: Option<&str>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(context))
    }
}

pub struct Game {
    config: GameConfiguration,
    menu: Menu,
}

impl Game {
    pub fn new(mut context: PluginContext) -> Self {
        let config = config::load_config();

        // Update resource loading to use the bsa resource loader
        {
            // Collect the archives that must be loaded
            let archives: Vec<&str> = config.archive.SArchiveList.split(", ").collect();
            let io = BsaResourceIo::load(&archives, context.resource_manager.resource_io());

            let mut state = context.resource_manager.state();
            state.set_resource_io(Arc::new(io));
        }

        // Create and set default font
        {
            let font_path = Path::new("textures/fonts/glow_monofonto_medium.fnt");
            let font = block_on(load_font(
                font_path,
                context.resource_manager.resource_io().as_ref(),
            ));

            // Set as default font.
            context.user_interface.default_font.set(font);

            set_default_font(context.user_interface);
        }

        let bytes = std::fs::read("Data/FalloutNV.esm").unwrap();

        let mut reader = Cursor::new(bytes);

        use binrw::BinRead;

        let plugin = espers::plugin::Plugin::parse(&mut reader).unwrap();

        let mut out = File::create("Data/FalloutNV.dump.esm").unwrap();

        let menu = block_on(Menu::new(&mut context, &config));

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
