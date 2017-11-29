use sfml::audio;
use std::thread;
use std::time::Duration;
use chan::Receiver;
use chan_signal::Signal;

use dmx_output::DmxOutput;
use error::Error;
use types::Runnable;
use types::runnable::{should_end, Status};

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

		Ok(Music { music: music })
	}
}

fn music_loop(music: &audio::Music, sigint: &Receiver<Signal>) -> Status {
    loop {
        if music.status() != audio::SoundStatus::Playing {
            return Status::Finished;
        }
        if should_end(sigint) {
            return Status::Interrupted;
        }

        thread::sleep(Duration::from_millis(15));
    }
}

impl <D: DmxOutput> Runnable<D> for Music {
	/// Run the playlist item
	#[allow(unused_variables)]
	fn run(&mut self, dmx: &mut D, sigint: &Receiver<Signal>) -> Result<Status, Error> {
		println!("Playing music");

        // Play music
        self.music.play();

        let status = music_loop(&self.music, sigint);

        // The music will stop automatically at the end. If it was paused some
        // how, stop to reset the playing position
        self.music.stop();
        Ok(status)
	}
}
