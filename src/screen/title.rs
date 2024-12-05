//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::game::assets::SoundtrackKey;
use crate::game::audio::soundtrack::PlaySoundtrack;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the poster texture
    let poster_handle = asset_server.load("images/poster.png");

    // Trigger the soundtrack
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Title));

    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            // Add the poster image
            children
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute, // Make it an absolutely positioned node
                        width: Val::Percent(100.0),            // Full width
                        height: Val::Percent(100.0),           // Full height
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE), // Transparent background for the container
                    ..Default::default()
                })
                .with_children(|poster| {
                    poster.spawn(ImageBundle {
                        image: UiImage::new(poster_handle),
                        style: Style {
                            width: Val::Percent(100.0), // Full width for image
                            height: Val::Auto,          // Maintain aspect ratio for height
                            ..Default::default()
                        },
                        z_index: ZIndex::Local(-1), // Ensure it's behind text and buttons
                        ..Default::default()
                    });
                });

            // Add the title and buttons
            children.title("Cycle of Valor");
            children.title_button("Play").insert(TitleAction::Play);
            children
                .title_button("Credits")
                .insert(TitleAction::Credits);
            #[cfg(not(target_family = "wasm"))]
            children.title_button("Exit").insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
