use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            ImageKey::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        )]
        .into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
    ClawSlash,
    SwordSlash,
    AxeSlash,
    ArrowFire,
    Hit,
    Whip,
    BloodSplatter,
    Health,
    BuildingPlacement,
    CoinPurchase,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        use SfxKey::*;

        let asset_server = world.resource::<AssetServer>();
        [
            (ButtonHover, asset_server.load("audio/sfx/button_hover.ogg")),
            (ButtonPress, asset_server.load("audio/sfx/button_press.ogg")),
            (Step1, asset_server.load("audio/sfx/step1.ogg")),
            (Step2, asset_server.load("audio/sfx/step2.ogg")),
            (Step3, asset_server.load("audio/sfx/step3.ogg")),
            (Step4, asset_server.load("audio/sfx/step4.ogg")),
            (ClawSlash, asset_server.load("audio/sfx/claw_slash.ogg")),
            (SwordSlash, asset_server.load("audio/sfx/sword_slash.ogg")),
            (AxeSlash, asset_server.load("audio/sfx/axe_slash.ogg")),
            (ArrowFire, asset_server.load("audio/sfx/arrow_fire.ogg")),
            (Hit, asset_server.load("audio/sfx/hit.ogg")),
            (Whip, asset_server.load("audio/sfx/whip.ogg")),
            (
                BloodSplatter,
                asset_server.load("audio/sfx/blood_splatter.ogg"),
            ),
            (Health, asset_server.load("audio/sfx/health.ogg")),
            (
                BuildingPlacement,
                asset_server.load("audio/sfx/building_placement.ogg"),
            ),
            (
                CoinPurchase,
                asset_server.load("audio/sfx/coin_purchase.ogg"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Title,
    Credits,
    Gameplay,
    Battle,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Title,
                asset_server.load("audio/soundtracks/demo_Beyond_Redemption_master.mp3"),
            ),
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/dova_Carousel_master.mp3"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/The_Enchanted_Forest_of_Min.mp3"),
            ),
            (
                SoundtrackKey::Battle,
                asset_server.load("audio/soundtracks/(Blood)_Stained_Glass_v0_8.mp3"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
