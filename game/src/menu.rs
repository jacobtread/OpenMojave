use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

use fyrox::{
    core::{color::Color, pool::Handle, reflect::*, visitor::*},
    gui::{
        border::BorderBuilder,
        brush::Brush,
        button::ButtonBuilder,
        decorator::{Decorator, DecoratorBuilder},
        define_widget_deref,
        grid::{Column, GridBuilder, Row},
        image::ImageBuilder,
        message::MessageDirection,
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        widget::{Widget, WidgetBuilder, WidgetMessage},
        BuildContext, Control, HorizontalAlignment, Thickness, UiNode, UserInterface,
        VerticalAlignment,
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

use crate::config::GameConfiguration;

pub struct Menu {
    scene: MenuScene,
    root: Handle<UiNode>,
    content: Handle<UiNode>,
}

struct MenuScene {
    scene: Handle<Scene>,
    music: Handle<Node>,
}

#[derive(Debug, Clone, Reflect, Visit)]
pub struct MenuButton {
    widget: Widget,
    border: Handle<UiNode>,
    text: Handle<UiNode>,
}

impl Deref for MenuButton {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for MenuButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

pub const MENU_COLOR: Color = Color::from_rgba(255, 182, 66, 255);

impl MenuButton {
    pub const BACKGROUND_DEFAULT: Color = Color::TRANSPARENT;
    pub const BACKGROUND_HOVER: Color = Color::from_rgba(255, 182, 66, 40);

    pub const BORDER_DEFAULT: Color = Color::TRANSPARENT;
    pub const BORDER_HOVER: Color = Color::from_rgba(255, 182, 66, 255);
}

impl Control for MenuButton {
    fn query_component(&self, type_id: std::any::TypeId) -> Option<&dyn std::any::Any> {
        if type_id == TypeId::of::<Self>() {
            Some(self)
        } else {
            None
        }
    }

    fn handle_routed_message(
        &mut self,
        ui: &mut UserInterface,
        message: &mut fyrox::gui::message::UiMessage,
    ) {
        self.widget.handle_routed_message(ui, message);

        // Only handle widget messages
        let msg = match message.data::<WidgetMessage>() {
            Some(value) => value,
            None => return,
        };

        // Ignore messages that aren't for us
        if message.destination() != self.handle() && !self.has_descendant(message.destination(), ui)
        {
            return;
        }

        match msg {
            WidgetMessage::MouseEnter => {
                ui.send_message(WidgetMessage::foreground(
                    self.border,
                    MessageDirection::ToWidget,
                    Brush::Solid(Self::BORDER_HOVER),
                ));

                ui.send_message(WidgetMessage::background(
                    self.border,
                    MessageDirection::ToWidget,
                    Brush::Solid(Self::BACKGROUND_HOVER),
                ));
            }

            WidgetMessage::MouseLeave => {
                ui.send_message(WidgetMessage::foreground(
                    self.border,
                    MessageDirection::ToWidget,
                    Brush::Solid(Self::BORDER_DEFAULT),
                ));

                ui.send_message(WidgetMessage::background(
                    self.border,
                    MessageDirection::ToWidget,
                    Brush::Solid(Self::BACKGROUND_DEFAULT),
                ));
            }

            _ => {}
        }
    }
}

pub struct MenuButtonBuilder {
    text: Option<String>,
}

impl MenuButtonBuilder {
    pub fn new() -> Self {
        Self { text: None }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn build(self, ctx: &mut BuildContext<'_>) -> Handle<UiNode> {
        let text = self.text.unwrap_or_default();
        let text = TextBuilder::new(
            WidgetBuilder::new()
                .with_margin(Thickness {
                    top: 4.,
                    bottom: 4.,
                    left: 12.,
                    right: 12.,
                })
                .with_foreground(Brush::Solid(MENU_COLOR)),
        )
        .with_text(text)
        .with_horizontal_text_alignment(HorizontalAlignment::Right)
        .with_vertical_text_alignment(VerticalAlignment::Center)
        .build(ctx);

        let border = BorderBuilder::new(
            WidgetBuilder::new()
                .with_child(text)
                .with_foreground(Brush::Solid(Color::TRANSPARENT))
                .with_background(Brush::Solid(MenuButton::BACKGROUND_DEFAULT)),
        )
        .with_stroke_thickness(Thickness::uniform(2.0))
        .build(ctx);

        let widget = WidgetBuilder::new()
            .with_margin(Thickness::uniform(2.))
            .with_width(340.)
            .with_vertical_alignment(VerticalAlignment::Stretch)
            .with_background(Brush::Solid(MenuButton::BACKGROUND_DEFAULT))
            .with_child(border)
            .build();

        let menu_button = MenuButton {
            border,
            text,
            widget,
        };

        ctx.add_node(UiNode::new(menu_button))
    }
}

impl Menu {
    pub async fn new(context: &mut PluginContext<'_, '_>, config: &GameConfiguration) -> Self {
        let scene = MenuScene::new(context).await;

        let screen_size = context.user_interface.screen_size();

        let ctx = &mut context.user_interface.build_ctx();

        let continue_btn = MenuButtonBuilder::new().with_text("Continue").build(ctx);
        let new_btn = MenuButtonBuilder::new().with_text("New").build(ctx);
        let load_btn = MenuButtonBuilder::new().with_text("Load").build(ctx);
        let settings_btn = MenuButtonBuilder::new().with_text("Settings").build(ctx);
        let credits_btn = MenuButtonBuilder::new().with_text("Credits").build(ctx);
        let dlc_btn = MenuButtonBuilder::new()
            .with_text("Downloadable Content")
            .build(ctx);
        let quit_btn = MenuButtonBuilder::new().with_text("Quit").build(ctx);

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
                                continue_btn,
                                new_btn,
                                load_btn,
                                settings_btn,
                                credits_btn,
                                dlc_btn,
                                quit_btn,
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
