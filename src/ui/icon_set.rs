use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    utils::HashMap,
};

pub struct IconSetUiPlugin;

impl Plugin for IconSetUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IconSet>()
            .add_systems(PreStartup, load_icon_set);
    }
}

fn load_icon_set(asset_server: Res<AssetServer>, mut icon_set: ResMut<IconSet>) {
    const PIXEL_ICONS: &[&str] = &[
        "gold_coins",
        "population",
        "selection_arrow",
        "attack_arrow",
    ];
    const ICONS: &[&str] = &[
        // Weapons
        "axe",
        "bandage",
        "bow",
        "claw_mark",
        "dagger",
        "mace",
        "sword",
        "whip",
        // Potions
        "fire_potion",
        "health_potion",
        "speed_potion",
        "strength_potion",
        // General
        "shop",
        "shop_character",
        "bg1",
        "bg2",
        "button1",
        "button1-gs",
    ];

    for pixel_icon in PIXEL_ICONS {
        info!("Loading pixel icon: {}", pixel_icon);
        let handle = asset_server.load_with_settings(
            format!("icons/{}.png", pixel_icon),
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            },
        );
        icon_set.insert(pixel_icon, handle);
    }

    for icon in ICONS {
        info!("Loading icon: {}", icon);
        let handle = asset_server.load(format!("icons/{}.png", icon));
        icon_set.insert(icon, handle);
    }
}

#[derive(Resource, Default)]
pub struct IconSet(HashMap<&'static str, Handle<Image>>);

impl IconSet {
    pub fn insert(&mut self, name: &'static str, handle: Handle<Image>) -> Option<Handle<Image>> {
        self.0.insert(name, handle)
    }

    /// Get cloned image handle.
    ///
    /// # Panic
    ///
    /// For ease of use, unwrap is used to panic if value does not exists for certain key.
    pub fn get(&self, name: &str) -> Handle<Image> {
        self.0.get(name).unwrap().clone()
    }
}
