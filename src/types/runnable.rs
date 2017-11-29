use dmx_output::DmxOutput;

use error::Error;
use chan::Receiver;
use chan_signal::Signal;

pub enum Status {
    Finished,
    Interrupted,
}

/// Interface for all types that can be run
pub trait Runnable<D: DmxOutput> {
	/// Run the item
	fn run(&mut self, dmx: &mut D, sigint: &Receiver<Signal>) -> Result<Status, Error>;
}

pub fn should_end(sigint: &Receiver<Signal>) -> bool {
    chan_select!{
        default => { return false },
        sigint.recv() -> _signal => { return true },
    }
}
