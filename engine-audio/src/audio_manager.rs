use std::collections::HashMap;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::io::Cursor;
use crate::sound::{Sound, SoundSettings};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundHandle(u64);

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: HashMap<SoundHandle, Arc<Mutex<Sink>>>,
    next_handle: u64,
    sounds: HashMap<String, Sound>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            sinks: HashMap::new(),
            next_handle: 0,
            sounds: HashMap::new(),
        })
    }

    pub fn load_sound(&mut self, name: String, sound: Sound) {
        self.sounds.insert(name, sound);
    }

    pub fn play(&mut self, name: &str, settings: SoundSettings) -> Option<SoundHandle> {
        let sound = self.sounds.get(name)?;
        
        let cursor = Cursor::new(sound.data.clone());
        let decoder = rodio::Decoder::new(cursor).ok()?;
        let source = decoder.speed(settings.speed);

        let sink = Sink::try_new(&self.stream_handle).ok()?;
        sink.set_volume(settings.volume);
        // Note: rodio's looping API has changed - looping disabled for now
        // To enable looping, you would need to use rodio's source looping methods
        sink.append(source);

        let handle = SoundHandle(self.next_handle);
        self.next_handle += 1;
        
        self.sinks.insert(handle, Arc::new(Mutex::new(sink)));
        
        Some(handle)
    }

    pub fn stop(&mut self, handle: SoundHandle) {
        if let Some(sink) = self.sinks.remove(&handle) {
            if let Ok(sink) = sink.lock() {
                sink.stop();
            }
        }
    }

    pub fn pause(&mut self, handle: SoundHandle) {
        if let Some(sink) = self.sinks.get(&handle) {
            if let Ok(sink) = sink.lock() {
                sink.pause();
            }
        }
    }

    pub fn resume(&mut self, handle: SoundHandle) {
        if let Some(sink) = self.sinks.get(&handle) {
            if let Ok(sink) = sink.lock() {
                sink.play();
            }
        }
    }

    pub fn set_volume(&mut self, handle: SoundHandle, volume: f32) {
        if let Some(sink) = self.sinks.get(&handle) {
            if let Ok(sink) = sink.lock() {
                sink.set_volume(volume.clamp(0.0, 1.0));
            }
        }
    }

    pub fn is_playing(&self, handle: SoundHandle) -> bool {
        if let Some(sink) = self.sinks.get(&handle) {
            if let Ok(sink) = sink.lock() {
                !sink.empty()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn cleanup_finished(&mut self) {
        let mut to_remove = Vec::new();
        
        for (handle, sink) in &self.sinks {
            if let Ok(sink) = sink.lock() {
                if sink.empty() {
                    to_remove.push(*handle);
                }
            }
        }
        
        for handle in to_remove {
            self.sinks.remove(&handle);
        }
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new().expect("Failed to create audio manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_manager_creation() {
        let manager = AudioManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_sound_handle() {
        let handle1 = SoundHandle(0);
        let handle2 = SoundHandle(1);
        assert_ne!(handle1, handle2);
    }
}
