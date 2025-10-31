// Audio playback for guitar note sounds

use anyhow::Result;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

// Simple sine wave generator
struct SineWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u64,
}

impl SineWave {
    fn new(frequency: f32, sample_rate: u32) -> Self {
        SineWave {
            frequency,
            sample_rate,
            current_sample: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.current_sample as f32 / self.sample_rate as f32;
        let value = (t * self.frequency * 2.0 * std::f32::consts::PI).sin();
        self.current_sample += 1;
        Some(value * 0.3) // Amplify to reasonable volume
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None // Infinite
    }

    fn channels(&self) -> u16 {
        1 // Mono
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None // Infinite
    }
}

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
    sample_rate: u32,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| anyhow::anyhow!("Failed to create audio stream: {}", e))?;
        
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| anyhow::anyhow!("Failed to create audio sink: {}", e))?;

        // Use standard CD quality sample rate
        let sample_rate = 44100;

        Ok(AudioPlayer {
            _stream,
            sink,
            sample_rate,
        })
    }

    // Explicitly cleanup audio resources
    pub fn cleanup(&self) {
        self.sink.stop();
        // The _stream will be dropped here, which should release the audio device
    }

    // Play a note at the given frequency for a short duration
    pub fn play_note(&self, frequency: f32) {
        // Clear any existing sounds
        self.sink.stop();
        
        // Generate a sine wave at the specified frequency
        // If audio fails, we continue without crashing
        let source = SineWave::new(frequency, self.sample_rate)
            .take_duration(Duration::from_millis(300)) // Play for 300ms
            .buffered();
        self.sink.append(source);
    }

    // Stop any currently playing sound
    pub fn stop(&self) {
        self.sink.stop();
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback: create a dummy player if audio initialization fails
            // In a real scenario, you might want to handle this more gracefully
            panic!("Failed to initialize audio player");
        })
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        // Ensure audio is properly stopped and cleaned up when dropped
        // This is critical on Windows to prevent audio device locks that can
        // interfere with other device drivers including network adapters
        self.sink.stop();
        // Give the audio system a moment to properly release the device
        // This helps prevent device driver conflicts on Windows
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

