// Original code taken from https://github.com/iliags/chip8/blob/main/crates/c8_audio/src/beeper.rs
#![allow(dead_code)]

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use std::sync::mpsc::{Receiver, Sender};

/// Beeper settings
#[derive(Debug, Clone, Copy)]
pub struct BeeperSettings {
    /// Tone pitch
    pub pitch: f32,

    /// Tone octave
    pub octave: f32,

    /// Tone volume
    pub volume: f32,
}

impl Default for BeeperSettings {
    fn default() -> Self {
        BeeperSettings {
            pitch: 440.0,
            octave: 2.0,
            volume: 0.5,
        }
    }
}

/// Messages for the beeper
#[derive(Debug, PartialEq)]
pub enum Message {
    /// Play the audio
    Play,
    /// Pause the audio
    Pause,
    /// Stop the audio
    Stop,
}

/// Beeper
#[derive(Default)]
pub struct Beeper {
    /// Sender used on non-wasm32 targets
    sender: Option<Sender<Message>>,

    stream: Option<Stream>,

    /// Beeper settings
    pub settings: BeeperSettings,

    pub is_playing: bool,
}

impl Clone for Beeper {
    fn clone(&self) -> Self {
        let mut beeper = Self::new(self.settings);
        beeper.settings = self.settings;
        beeper
    }
}

impl std::fmt::Debug for Beeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Beeper {{ sender: {:?} }}", self.sender)
    }
}

impl Beeper {
    /// Create a beeper instance
    pub fn new(beeper_settings: BeeperSettings) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::sync::mpsc;
            use std::thread;

            let mut beeper = Beeper::default();
            beeper.settings = beeper_settings;

            let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

            thread::spawn(move || {
                let device = Self::create_stream_device(beeper.settings);

                Self::stream_audio(receiver, device);
            });

            beeper.sender = Some(sender);

            beeper
        }
    }

    /// Play the audio
    pub fn play(&mut self) {
        self.is_playing = true;

        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Play),
            None => Ok(()),
        };
    }

    //noinspection DuplicatedCode
    /// Pause the audio
    pub fn pause(&mut self) {
        self.is_playing = false;

        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Pause),
            None => Ok(()),
        };
    }

    //noinspection DuplicatedCode
    /// Stop the audio
    pub fn stop(&mut self) {
        self.is_playing = false;

        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Stop),
            None => Ok(()),
        };
    }

    fn create_stream_device(settings: BeeperSettings) -> Result<Stream, BuildStreamError> {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device
            .default_output_config()
            .expect("no default output config");

        match config.sample_format() {
            cpal::SampleFormat::F32 => {
                Self::create_stream::<f32>(&device, &config.into(), settings)
            }
            cpal::SampleFormat::I16 => {
                Self::create_stream::<i16>(&device, &config.into(), settings)
            }
            cpal::SampleFormat::U16 => {
                Self::create_stream::<u16>(&device, &config.into(), settings)
            }
            sample_format => Err(BuildStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: format!("Unsupported sample format '{sample_format}'"),
                },
            }),
        }
    }

    fn stream_audio(receiver: Receiver<Message>, stream: Result<Stream, BuildStreamError>) {
        match stream {
            Ok(stream) => loop {
                match receiver.recv() {
                    Ok(Message::Play) => {
                        let _ = stream.play();
                    }
                    Ok(Message::Pause) => {
                        let _ = stream.pause();
                    }
                    Ok(Message::Stop) => {
                        let _ = stream.pause();
                        return;
                    }
                    Err(e) => {
                        eprintln!("Receive error: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("BuildStreamError {:?}", e);
            }
        }
    }

    fn create_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        settings: BeeperSettings,
    ) -> Result<Stream, BuildStreamError>
    where
        T: SizedSample + FromSample<f32>,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * settings.pitch * settings.octave * std::f32::consts::PI / sample_rate)
                .sin()
                * settings.volume
        };

        let err_fn = |err| eprintln!("Stream error: {}", err);

        device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Self::write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: Sample + FromSample<f32>,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = T::from_sample(next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}

impl Drop for Beeper {
    fn drop(&mut self) {
        if let Some(ref sender) = self.sender {
            sender.send(Message::Stop).unwrap_or_else(|e| {
                eprintln!("Error sending stop message: {}", e);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_beeper() {
        let mut beeper = Beeper::new(BeeperSettings::default());

        beeper.play();
        std::thread::sleep(std::time::Duration::from_secs(1));
        beeper.pause();
        std::thread::sleep(std::time::Duration::from_secs(1));
        beeper.play();
        std::thread::sleep(std::time::Duration::from_secs(1));
        beeper.stop();
    }
}
