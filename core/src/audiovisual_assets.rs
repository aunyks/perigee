use serde::{Deserialize, Serialize};

const ASSET_TYPE_OFFSET: u32 = u32::MAX / 2;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Animation {
    CameraIdle,
    CameraRunning,
}

impl From<Animation> for u32 {
    fn from(anim: Animation) -> Self {
        match anim {
            Animation::CameraIdle => 0,
            Animation::CameraRunning => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Sound {
    PlayerSighRelaxed,
}

impl From<Sound> for u32 {
    fn from(sound: Sound) -> Self {
        match sound {
            Sound::PlayerSighRelaxed => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Asset {
    Animation(Animation),
    Sound(Sound),
}

impl From<Asset> for u32 {
    fn from(asset: Asset) -> Self {
        match asset {
            #[allow(clippy::erasing_op)]
            Asset::Sound(sound) => {
                let sound_id: u32 = sound.into();
                (ASSET_TYPE_OFFSET * 0) + sound_id
            }
            #[allow(clippy::identity_op)]
            Asset::Animation(animation) => {
                let animation_id: u32 = animation.into();
                (ASSET_TYPE_OFFSET * 1) + animation_id
            }
        }
    }
}
