#[cfg(feature = "dev")]
mod dev_tools;
mod game;
pub mod path_finding;
mod screen;
mod ui;

const BASE_APP_HEIGHT: f32 = 720.0;
const BASE_CAM_SCALE: f32 = 3.2;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
    window::PrimaryWindow,
};
use game::tile_set::TILE_HALF_HEIGHT;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Jam 5".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, ui::plugin))
            .add_systems(Update, update_camera_scale);

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(Component)]
pub struct VillageCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((VillageCamera,
        Name::new("Camera"),
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 3.2,
                ..default()
            },
            ..default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}

fn update_camera_scale(
    windows: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    mut projections: Query<&mut OrthographicProjection, With<IsDefaultUiCamera>>,
) {
    let (Ok(window), Ok(mut projection)) = (windows.get_single(), projections.get_single_mut())
    else {
        return;
    };

    let window_height = window.size().y;

    if window_height > f32::EPSILON {
        let scale = BASE_APP_HEIGHT / window_height * BASE_CAM_SCALE;
        projection.scale = scale;
    }
}
