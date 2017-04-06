use DmxOutput;
use error::Error;

/// Interface for all types that can be run
pub trait Runnable {
	/// Run the item
	fn run(self: Box<Self>, dmx: &mut DmxOutput) -> Result<(), Error>;
}
