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
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        widget::{WidgetBuilder, WidgetMessage},
        HorizontalAlignment, Thickness, UiNode, UserInterface, VerticalAlignment,
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

        let mut menu_button = |text: &str, row: usize| -> Handle<UiNode> {
            ButtonBuilder::new(
                WidgetBuilder::new()
                    .on_row(row)
                    .with_horizontal_alignment(HorizontalAlignment::Right)
                    .with_vertical_alignment(VerticalAlignment::Stretch),
            )
            .with_back(
                DecoratorBuilder::new(
                    BorderBuilder::new(
                        WidgetBuilder::new()
                            .with_foreground(Brush::Solid(Color::RED))
                            .with_child(
                                TextBuilder::new(WidgetBuilder::new())
                                    .with_text(text)
                                    .with_horizontal_text_alignment(HorizontalAlignment::Left)
                                    .with_vertical_text_alignment(VerticalAlignment::Center)
                                    .build(ctx),
                            ),
                    )
                    .with_stroke_thickness(Thickness::uniform(1.0)),
                )
                .with_normal_brush(Brush::Solid(Color::TRANSPARENT))
                .with_hover_brush(Brush::Solid(Color::TRANSPARENT))
                .with_pressed_brush(Brush::Solid(Color::TRANSPARENT))
                .build(ctx),
            )
            .build(ctx)
        };

        let continue_button = menu_button("Continue", 0);
        let new_button = menu_button("New", 1);
        let load_button = menu_button("Load", 2);
        let settings_button = menu_button("Settings", 3);
        let credits_button = menu_button("Credits", 4);
        let dlc_button = menu_button("Downloadable Content", 5);
        let quit_button = menu_button("Quit", 6);

        let content = GridBuilder::new(
            WidgetBuilder::new()
                .with_margin(Thickness::uniform(15.))
                .with_children([
                    // Title image
                    ImageBuilder::new(
                        WidgetBuilder::new()
                            .with_horizontal_alignment(HorizontalAlignment::Left)
                            .with_width(512.)
                            .with_height(128.),
                    )
                    .with_texture(into_gui_texture(
                        context
                            .resource_manager
                            .request::<Texture, _>("textures/interface/main/main_title.dds"),
                    ))
                    .build(ctx),
                    // Menu Buttons
                    StackPanelBuilder::new(
                        WidgetBuilder::new()
                            .with_vertical_alignment(VerticalAlignment::Center)
                            .with_horizontal_alignment(HorizontalAlignment::Right)
                            .on_column(1)
                            .with_children([
                                continue_button,
                                new_button,
                                load_button,
                                settings_button,
                                credits_button,
                                dlc_button,
                                quit_button,
                            ]),
                    )
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
                .request::<Texture, _>("textures/interface/main/main_background.dds"),
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
