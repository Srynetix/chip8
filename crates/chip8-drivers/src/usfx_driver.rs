use std::sync::{Arc, Mutex};

use chip8_core::drivers::AudioInterface;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

pub struct UsfxAudioDriver {
    thread: AudioThread,
}

impl Default for UsfxAudioDriver {
    fn default() -> Self {
        let mut thread = AudioThread::default();
        thread.run();

        Self { thread }
    }
}

impl AudioInterface for UsfxAudioDriver {
    fn play_beep(&mut self) {
        self.thread.play_beep();
    }
}

struct AudioThread {
    mixer: Arc<Mutex<usfx::Mixer>>,
}

impl Default for AudioThread {
    fn default() -> Self {
        Self {
            mixer: Arc::new(Mutex::new(usfx::Mixer::new(44_100))),
        }
    }
}

impl AudioThread {
    pub fn play(&mut self, sample: usfx::Sample) {
        self.mixer.lock().unwrap().play(sample);
    }

    pub fn play_beep(&mut self) {
        let mut sample = usfx::Sample::default();
        sample.volume(0.5);
        sample.osc_frequency(500);
        sample.osc_type(usfx::OscillatorType::Sine);
        sample.env_attack(0.0);
        sample.env_decay(0.1);
        sample.env_sustain(0.5);
        sample.env_release(0.5);
        sample.dis_crunch(0.5);
        self.play(sample);
    }

    pub fn run(&mut self) {
        let mixer = self.mixer.clone();

        let host = cpal::default_host();
        let event_loop = host.event_loop();
        let device = host.default_output_device().unwrap();
        let fmt = device.default_output_format().unwrap();

        let stream_id = event_loop.build_output_stream(&device, &fmt).unwrap();
        event_loop.play_stream(stream_id).unwrap();

        std::thread::spawn(move || {
            event_loop.run(move |_, stream_result| {
                let stream_data = match stream_result {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("error on stream: {:?}", err);
                        return;
                    }
                };

                match stream_data {
                    cpal::StreamData::Output {
                        buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                    } => {
                        mixer.lock().unwrap().generate(&mut buffer);
                    }
                    _ => panic!("Unusable output buffer"),
                }
            });
        });
    }
}
