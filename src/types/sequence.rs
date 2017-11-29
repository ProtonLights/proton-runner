use sfml::audio;
use std::thread;
use std::time::Duration;

use dmx_output::DmxOutput;
use error::Error;
use types::{Runnable, SequenceData};
use utils;

pub struct Sequence {
	seq_data: SequenceData,
	music: audio::Music
}

impl Sequence {
	pub fn new(seq_path: String, music_path: String) -> Result<Sequence, Error> {
		// TODO check if paths exist
		let data = try!(utils::load_sequence_data(&seq_path));
        let music = match audio::Music::from_file(&music_path) {
            Some(mm) => mm,
            None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
        };

		Ok(Sequence {
			seq_data: data,
			music: music
		})
	}
}

impl <D: DmxOutput> Runnable<D> for Sequence {
	/// Run the playlist item
	fn run(&mut self, dmx: &mut D) -> Result<(), Error> {
		println!("Running sequence");

        let num_frames = self.seq_data.num_frames;
        let music_dur = self.music.duration().as_milliseconds();
        let music_frame_dur = music_dur as f32 / num_frames as f32;

        // Play music
        self.music.play();

        loop {            
            let frame = (self.music.playing_offset().as_milliseconds() as f32 / music_frame_dur) as u32;

            if frame < num_frames {
                let d = &self.seq_data.data[frame as usize];
                match dmx.send(d) {
                    Ok(_) => (),
                    Err(e) => println!("\tError: {}", e),
                }
            }

            // Stop when music done or past last frame
            if self.music.status() == audio::SoundStatus::Stopped {
                break;
            }

            // Sleep for frame duration
            thread::sleep(Duration::from_millis(15));
        }

        println!("Done.");
        Ok(())
	}
}
