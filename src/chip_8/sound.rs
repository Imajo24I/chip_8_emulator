use rodio::queue;
use rodio::source::SineWave;
use rodio::{OutputStream, Sink};

use std::sync::mpsc::{self, Receiver, Sender};

pub enum Command {
    Play,
    Pause,
    Stop,
    Sync(f32),
}

#[derive(Clone)]
pub struct Beeper {
    tx: Sender<Command>,
    volume: f32,
}

impl Default for Beeper {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self::spawn_thread(rx);

        Self {
            tx,
            volume: Self::DEFAULT_VOLUME,
        }
    }
}

impl Beeper {
    const DEFAULT_VOLUME: f32 = 0.05;
    const BEEP_FREQ: f32 = 440.0;

    pub fn play(&mut self) {
        self.tx.send(Command::Play).unwrap();
    }

    pub fn pause(&mut self) {
        self.tx.send(Command::Pause).unwrap();
    }

    pub fn stop(&mut self) {
        self.tx.send(Command::Stop).unwrap();
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.tx.send(Command::Sync(volume)).unwrap();
    }

    fn spawn_thread(rx: Receiver<Command>) {
        std::thread::spawn(move || {
            let (queue, output_queue) = queue::queue(true);

            if let Ok((_steam, stream_handle)) = OutputStream::try_default() {
                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.append(output_queue);
                sink.pause();
                sink.set_volume(Self::DEFAULT_VOLUME);
                queue.append(SineWave::new(Self::BEEP_FREQ));

                loop {
                    if let Ok(cmd) = rx.recv() {
                        match cmd {
                            Command::Play => sink.play(),
                            Command::Pause => sink.pause(),
                            Command::Sync(volume) => sink.set_volume(volume),

                            Command::Stop => {
                                sink.stop();
                                break;
                            }
                        }
                    }
                }
            }
        });
    }
}
