use std::thread;
use std::time::Duration;
use chan::Receiver;
use chan_signal::Signal;

use commands;
use dmx_output::DmxOutput;
use error::Error;
use types::Runnable;
use types::runnable::{should_end, Status};

pub struct Delay {
	duration_ms: u32
}

impl Delay {
	pub fn new(duration_ms: u32) -> Result<Delay, Error> {
		Ok(Delay {
			duration_ms: duration_ms
		})
	}
}

impl <D: DmxOutput> Runnable<D> for Delay {
	/// Run the playlist item
	fn run(&mut self, dmx: &mut D, sigint: &Receiver<Signal>) -> Result<Status, Error> {
		println!("Playing delay");

    	commands::all_off(dmx)?;

        // We'll short a little, but never more than 15 ms.
        let steps = self.duration_ms / 15;

        for _ in 0..steps {
            if should_end(sigint) {
                return Ok(Status::Interrupted);
            }

            thread::sleep(Duration::from_millis(self.duration_ms as u64));
        }
        
        Ok(Status::Finished)
	}
}
