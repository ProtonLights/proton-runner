use std::thread;
use std::time::Duration;

use commands;
use DmxOutput;
use error::Error;
use types::Runnable;

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

impl Runnable for Delay {
	/// Run the playlist item
	fn run(self: Box<Self>, dmx: &mut DmxOutput) -> Result<(), Error> {
		println!("Playing delay");

    	try!(commands::all_off(dmx));
        thread::sleep(Duration::from_millis(self.duration_ms as u64));
        
        Ok(())
	}
}
