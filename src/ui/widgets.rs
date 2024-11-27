//! Helper traits for creating common widgets.

use bevy::{
    ecs::{system::EntityCommands, world::Command},
    prelude::*,
    ui::Val::*,
};

use super::{interaction::InteractionPalette, palette::*};

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;

    // Spawn a big title in gothic font
    fn title(&mut self, text: impl Into<String>) -> EntityCommands;

    // Title menu buttons
    fn title_button(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn + SpawnCommand> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(100.0),
                    height: Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(4.0)),
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 20.0,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn title_button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(100.0),
                    height: Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(4.0)),
                background_color: BackgroundColor(TITLE_BUTTON_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: TITLE_BUTTON_BACKGROUND,
                hovered: TITLE_BUTTON_HOVERED_BACKGROUND,
                pressed: TITLE_BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 20.0,
                        color: TITLE_BUTTON_TEXT_COLOR,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            NodeBundle {
                style: Style {
                    width: Px(250.0),
                    height: Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Header Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 20.0,
                        color: HEADER_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Px(250.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Label Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 18.0,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn title(&mut self, text: impl Into<String>) -> EntityCommands {
        let text: String = text.into();
        self.spawn_command(|id| {
            let title = text.clone();
            move |world: &mut World| {
                world.resource_scope::<AssetServer, _>(|world, asset_server| {
                    world
                        .entity_mut(id)
                        .insert((
                            Name::new("Title"),
                            NodeBundle {
                                style: Style {
                                    width: Px(250.0),
                                    height: Px(32.0),
                                    margin: UiRect::all(Px(15.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(move |children| {
                            children.spawn((
                                Name::new("Title Text"),
                                TextBundle::from_section(
                                    title,
                                    TextStyle {
                                        font_size: 40.0,
                                        color: TITLE_TEXT_COLOR,
                                        font: asset_server.load(TITLE_TEXT_FONT_PATH),
                                    },
                                ),
                            ));
                        });
                });
            }
        })
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(5.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

trait SpawnCommand {
    fn spawn_command<C>(&mut self, f: impl Fn(Entity) -> C) -> EntityCommands
    where
        C: Command;
}

impl SpawnCommand for Commands<'_, '_> {
    fn spawn_command<C>(&mut self, f: impl Fn(Entity) -> C) -> EntityCommands
    where
        C: Command,
    {
        let mut entity_commands = self.spawn_empty();
        let id = entity_commands.id();
        entity_commands.commands().add(f(id));
        entity_commands
    }
}

impl SpawnCommand for ChildBuilder<'_> {
    fn spawn_command<C>(&mut self, f: impl Fn(Entity) -> C) -> EntityCommands
    where
        C: Command,
    {
        let mut entity_commands = self.spawn_empty();
        let id = entity_commands.id();
        entity_commands.commands().add(f(id));
        entity_commands
    }
}
