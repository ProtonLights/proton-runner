use dmx_output::DmxOutput;
use error::Error;

/// Interface for all types that can be run
pub trait Runnable<D: DmxOutput> {
	/// Run the item
	fn run(&mut self, dmx: &mut D) -> Result<(), Error>;
}
