use sfml::audio;
use std::thread;
use std::time::Duration;

use dmx_output::DmxOutput;
use error::Error;
use types::Runnable;

pub struct Music {
	music: audio::Music
}

impl Music {
	pub fn new(music_path: String) -> Result<Music, Error> {
		// TODO check if path exists
        let music = match audio::Music::from_file(&music_path) {
            Some(mm) => mm,
            None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
        };

		Ok(Music {
			music: music
		})
	}
}

impl <D: DmxOutput> Runnable<D> for Music {
	/// Run the playlist item
	#[allow(unused_variables)]
	fn run(&mut self, dmx: &mut D) -> Result<(), Error> {
		println!("Playing music");

        // Play music
        self.music.play();

        // Loop until done playing
        while self.music.status() == audio::SoundStatus::Playing {
            // Leave some CPU time for other processes
            thread::sleep(Duration::from_millis(15));
        }

        // The music will stop automatically at the end. If it was paused some
        // how, stop to reset the playing position
        self.music.stop();

        Ok(())
	}
}
