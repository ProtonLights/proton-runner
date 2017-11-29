use std::thread;
use std::time::Duration;
use chan::Receiver;
use chan_signal::Signal;

use dmx_output::DmxOutput;
use error::Error;
use types::{Runnable, SequenceData};
use types::runnable::{should_end, Status};
use utils;


pub struct Pattern {
	data: SequenceData
}

impl Pattern {
	pub fn new(seq_path: String) -> Result<Pattern, Error> {
		// TODO check if path exists
        let data = try!(utils::load_sequence_data(&seq_path));

		Ok(Pattern { data: data })

	}
}

impl <D: DmxOutput> Runnable<D> for Pattern {
	/// Run the playlist item
	fn run(&mut self, dmx: &mut D, sigint: &Receiver<Signal>) -> Result<Status, Error> {
		println!("Running pattern");

        let frame_dur = self.data.frame_dur_ms as u64;

        for frame in &self.data.data {
            if should_end(sigint) {
                return Ok(Status::Interrupted);
            }

            match dmx.send(frame) {
                Ok(_) => (),
                Err(e) => println!("\tError: {}", e),
            }
            thread::sleep(Duration::from_millis(frame_dur));
        }

        println!("Done.");
        Ok(Status::Finished)
	}
}
