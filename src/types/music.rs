use sfml::audio;
use sfml::system::{Time, sleep};

use DmxOutput;
use error::Error;
use types::Runnable;

pub struct Music {
	music: audio::Music
}

impl Music {
	pub fn new(music_path: String) -> Result<Music, Error> {
		// TODO check if path exists
        let music = match audio::Music::new_from_file(&music_path) {
            Some(mm) => mm,
            None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
        };

		Ok(Music {
			music: music
		})
	}
}

impl Runnable for Music {
	/// Run the playlist item
	#[allow(unused_variables)]
	fn run(self: Box<Self>, dmx: &mut DmxOutput) -> Result<(), Error> {
		println!("Playing music");
		let mut music = self.music;

        // Play music
        music.play();

        // Loop until done playing
        while music.get_status() == audio::SoundStatus::Playing {
            // Leave some CPU time for other processes
            sleep(Time::with_milliseconds(100));
        }

        Ok(())
	}
}
