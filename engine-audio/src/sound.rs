use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoundSettings {
    pub volume: f32,
    pub speed: f32,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            speed: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub data: Vec<u8>,
    pub channels: u16,
    pub sample_rate: u32,
}

impl Sound {
    pub fn new(data: Vec<u8>, channels: u16, sample_rate: u32) -> Self {
        Self {
            data,
            channels,
            sample_rate,
        }
    }

    pub fn load_wav<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        
        // Simple WAV parsing (simplified - in production use a proper WAV library)
        // For now, we'll assume the file is already in a format rodio can handle
        Ok(Self {
            data,
            channels: 2,
            sample_rate: 44100,
        })
    }

    pub fn duration(&self) -> f32 {
        let bytes_per_sample = 2; // 16-bit
        let total_samples = self.data.len() / (self.channels as usize * bytes_per_sample);
        total_samples as f32 / self.sample_rate as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_creation() {
        let sound = Sound::new(vec![0u8; 100], 2, 44100);
        assert_eq!(sound.channels, 2);
        assert_eq!(sound.sample_rate, 44100);
    }

    #[test]
    fn test_sound_settings_default() {
        let settings = SoundSettings::default();
        assert_eq!(settings.volume, 1.0);
        assert_eq!(settings.speed, 1.0);
    }
}
