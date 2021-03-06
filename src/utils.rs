use std::fs::File;
use std::io::Read;
use std::path::Path;

use rustc_serialize::json;

use error::Error;
use types::SequenceData;


/// Get sequence data from file
pub fn load_sequence_data(path: &str) -> Result<SequenceData, Error> {

    println!("\tReading sequence data from file");

    // Read sequence data
    let data_json = try!(file_as_string(path));
    let data: SequenceData = try!(json::decode(&data_json).map_err(Error::JsonDecode));

    // Make sure there is data for each frame
    if data.data.len() != data.num_frames as usize {
        Err(Error::InvalidDataLength(data.data.len() as u32, data.num_frames))
    } else {
        Ok(data)
    }
}


/// Transposes data from channel-major to frame-major.
/// Assuming rectangle vector
pub fn transpose_data(data: Vec<Vec<u16>>) -> Result<Vec<Vec<u16>>, Error> {
    if data.len() == 0 || data[0].len() == 0 {
        Err(Error::EmptyData)
    } else {
        let num_frames = data[0].len();
        let num_channels = data.len();
        
        let mut transposed = vec![Vec::with_capacity(num_channels as usize); num_frames as usize];
        for channel_data in data.iter() {
            for (frame_idx, frame_data) in channel_data.iter().enumerate() {
                transposed[frame_idx].push(*frame_data);
            }
        }
        Ok(transposed)
    }
}

/// Checks that a path exists
pub fn check_path(path: &str) -> Result<(), Error> {
    if !Path::new(path).exists() {
        Err(Error::PathNotFound(path.to_string()))
    } else {
        Ok(())
    }
}

/// Reads a file as a string.
/// Wraps Read::read_to_string errors in proton_cli::Error
pub fn file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    if !path.as_ref().exists() {
        return Err(Error::PathNotFound(path.as_ref().to_str().expect("Path not valid UTF-8").to_string()));
    }
    File::open(path.as_ref())
        .and_then(|mut file| {
            let mut string = String::new();
            file.read_to_string(&mut string)
                .and_then(|_| Ok(string.trim().to_string()))           
        })
        .map_err(Error::Io)
}
