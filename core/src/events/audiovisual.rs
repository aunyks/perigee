use crate::audiovisual_assets::Asset;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AudioVisualOperation {
    Play,
    Pause,
    Stop,
    Loop,
    FadeIn,
    FadeOut,
    Clear,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AudioVisualEvent {
    asset: Asset,
    operation: AudioVisualOperation,
}

impl From<AudioVisualEvent> for u32 {
    fn from(event: AudioVisualEvent) -> Self {
        match event.operation {
            AudioVisualOperation::Play => 0,
            AudioVisualOperation::Pause => 1,
            AudioVisualOperation::Stop => 2,
            AudioVisualOperation::Loop => 3,
            AudioVisualOperation::FadeIn => 4,
            AudioVisualOperation::FadeOut => 5,
            AudioVisualOperation::Clear => 6,
        }
    }
}

impl AudioVisualEvent {
    pub fn new(operation: AudioVisualOperation, asset: Asset) -> Self {
        Self { asset, operation }
    }
    pub fn asset(&self) -> &Asset {
        &self.asset
    }
}
