use fyrox::{
    core::{color::Color, pool::Handle},
    gui::{
        border::BorderBuilder,
        brush::Brush,
        button::ButtonBuilder,
        decorator::{Decorator, DecoratorBuilder},
        grid::{Column, GridBuilder, Row},
        image::ImageBuilder,
        message::MessageDirection,
        widget::{WidgetBuilder, WidgetMessage},
        HorizontalAlignment, UiNode, UserInterface, VerticalAlignment,
    },
    plugin::PluginContext,
    resource::texture::Texture,
    scene::{
        base::BaseBuilder,
        node::Node,
        sound::{SoundBuffer, SoundBuilder, Status},
        Scene,
    },
    utils::into_gui_texture,
};

pub struct Menu {
    scene: MenuScene,
    root: Handle<UiNode>,
    content: Handle<UiNode>,
}

struct MenuScene {
    scene: Handle<Scene>,
    music: Handle<Node>,
}

impl Menu {
    pub async fn new(context: &mut PluginContext<'_, '_>) -> Self {
        let scene = MenuScene::new(context).await;

        let screen_size = context.user_interface.screen_size();

        let ctx = &mut context.user_interface.build_ctx();

        let button_dec = DecoratorBuilder::new(BorderBuilder::new(WidgetBuilder::new()))
            .with_normal_brush(Brush::Solid(Color::TRANSPARENT))
            .build(ctx);

        let content = GridBuilder::new(
            WidgetBuilder::new().with_children([
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_column(0)
                        .with_width(512.)
                        .with_height(128.),
                )
                .with_texture(into_gui_texture(
                    context.resource_manager.request::<Texture, _>(
                        "DataUnpacked/textures/interface/main/main_title.dds",
                    ),
                ))
                .build(ctx),
                GridBuilder::new(
                    WidgetBuilder::new()
                        .with_horizontal_alignment(HorizontalAlignment::Stretch)
                        .on_column(1)
                        .with_children([
                            // Continue
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(0))
                                .with_text("Continue")
                                .build(ctx),
                            // New
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(1))
                                .with_text("New")
                                .build(ctx),
                            // Load
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(2))
                                .with_text("Load")
                                .build(ctx),
                            // Settings
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(3))
                                .with_text("Settings")
                                .build(ctx),
                            // Credits
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(4))
                                .with_text("Credits")
                                .build(ctx),
                            // Downloadable Content
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(5))
                                .with_text("Downloadable Content")
                                .build(ctx),
                            // Quit
                            ButtonBuilder::new(WidgetBuilder::new().on_column(0).on_row(6))
                                .with_text("Quit")
                                .build(ctx),
                        ]),
                )
                .add_column(Column::stretch())
                .add_row(Row::auto())
                .add_row(Row::auto())
                .add_row(Row::auto())
                .add_row(Row::auto())
                .add_row(Row::auto())
                .add_row(Row::auto())
                .add_row(Row::auto())
                .build(ctx),
            ]),
        )
        .add_row(Row::stretch())
        .add_column(Column::stretch())
        .add_column(Column::stretch())
        .build(ctx);

        let background = ImageBuilder::new(
            WidgetBuilder::new()
                .with_width(screen_size.x)
                .with_height(screen_size.y)
                .with_child(content),
        )
        .with_texture(into_gui_texture(
            context
                .resource_manager
                .request::<Texture, _>("DataUnpacked/textures/interface/main/main_background.dds"),
        ))
        .build(ctx);
        // Create some widgets as usual.

        Self {
            scene,
            root: background,
            content,
        }
    }

    pub fn resize(&self, ui: &UserInterface, width: f32, height: f32) {
        ui.send_message(WidgetMessage::width(
            self.root,
            MessageDirection::ToWidget,
            width,
        ));
        ui.send_message(WidgetMessage::height(
            self.root,
            MessageDirection::ToWidget,
            height,
        ));
    }
}

impl MenuScene {
    pub async fn new(context: &mut PluginContext<'_, '_>) -> Self {
        let mut scene = Scene::new();

        let buffer = context
            .resource_manager
            .request::<SoundBuffer, _>("Data/MainTitle.wav")
            .await
            .unwrap();

        let music = SoundBuilder::new(BaseBuilder::new())
            .with_buffer(buffer.into())
            .with_looping(true)
            .with_status(Status::Playing)
            .build(&mut scene.graph);

        let scene = context.scenes.add(scene);

        Self { scene, music }
    }
}
