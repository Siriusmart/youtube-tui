use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdaptiveFormat {
    #[serde(default)]
    pub index: String,
    #[serde(default)]
    pub bitrate: String,
    #[serde(default)]
    pub init: String,
    pub url: String,
    pub itag: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub clen: String,
    #[serde(default)]
    pub lmt: String,
    #[serde(default)]
    #[serde(rename = "protectionType")]
    pub projection_type: String,
    #[serde(default)]
    pub fps: u8,
    #[serde(default)]
    pub container: String,
    #[serde(default)]
    pub encoding: String,
    #[serde(default)]
    #[serde(rename = "quality")]
    pub quality: String,
    #[serde(default)]
    pub resolution: String,
    #[serde(default)]
    #[serde(rename = "qualityLabel")]
    pub quality_label: String,
    #[serde(default)]
    #[serde(rename = "audioQuality")]
    pub audio_quality: String,
    #[serde(default)]
    #[serde(rename = "audioSampleRate")]
    pub audio_sample_rate: usize,
    #[serde(default)]
    #[serde(rename = "audioChannels")]
    pub audio_channels: usize,
}
