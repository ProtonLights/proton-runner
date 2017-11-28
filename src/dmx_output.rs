use std::io::Write;

use error::Error;
use rserial;

pub struct DmxOutput {
    serial: rserial::SystemPort
}


impl DmxOutput {
    /// Create a new Dmx outputter for a device located at port
    pub fn new(port: &str) -> Result<DmxOutput, Error> {
        let serial = try!(rserial::open(port).map_err(Error::Serial));
        Ok(DmxOutput {
            serial: serial
        })
    }

    /// Formulate frame to send to Enttec USB DMX Pro
    fn send_msg(&mut self, label: u8, message: &Vec<u8>) -> Result<(), Error> {
        // How many data points to send
        let length = message.len();
        let lm = length >> 8;
        let ll = length - (lm << 8);
        if length > 600 {
            Err(Error::DmxTooLong(length))
        } else {
            // Create the array to write to the serial port
            let mut output = vec![0x7E, label, ll as u8, lm as u8];
            output.extend(message.iter().cloned());
            output.push(0xE7);
            // Convert to byte array and write it
            let _ = try!(self.serial.by_ref().write(&output).map_err(Error::Io));
            try!(self.serial.by_ref().flush().map_err(Error::Io));
            Ok(())
        }
    }

    /// Send array of up to 512 channels to DMX
    pub fn send(&mut self, values: &Vec<u16>) -> Result<(), Error> {
        if values.len() > 513 { // TODO: Multiple universes
            println!("More than 512 dmx channels given ({}); ignoring all past 512", values.len());
        }

        // Make values u8
        let mut values_u8 = values.iter()
            .skip(1) // Skip dmx channel 0 (doesn't exist)
            .map(|val| *val as u8)
            .collect::<Vec<u8>>();

        // Limit to 512 channels for now
        values_u8.truncate(512);

        // First value is always 0 (DMX starts at 1)
        let mut data = vec![0];
        data.append(&mut values_u8);
        
        // Fill up the channels up to the minimum of 25
        while data.len() < 25 {
            // Make all the extra channels 0
            data.push(0);
        }
        
        // Send all the data with label 6
        self.send_msg(6, &data)
    }
}
