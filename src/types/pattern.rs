use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dmx_output::DmxOutput;
use error::Error;
use types::{Runnable, SequenceData};
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
	fn run(&mut self, dmx: &mut D) -> Result<(), Error> {
		println!("Running pattern");

        // Create channels for clock thread tx/rx
        let (tx, rx) = mpsc::channel();

        // Spawn timer that ticks once per frame until all frames have been ticked
        let num_frames = self.data.num_frames;
        let frame_dur = self.data.frame_dur_ms as u64;
        let mut curr_frame = 0;
        thread::spawn(move || {
            while curr_frame != num_frames {
                // TODO maybe map the unwrap error to Error type
                tx.send(curr_frame).unwrap();
                curr_frame += 1;
                thread::sleep(Duration::from_millis(frame_dur));
            }
            
        });

        // Output every frame
        for frame in rx {
            let d = &self.data.data[frame as usize];
            match dmx.send(d) {
                Ok(_) => (),
                Err(e) => println!("\tError: {}", e),
            }
        }
        println!("Done.");
        Ok(())
	}
}
