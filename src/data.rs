use rustc_serialize::json;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::str;

use error::Error;
use types::{Config, Playlist, PlaylistItem, SequenceData};
use utils;


/// Get the sequence data from proton_cli
fn get_data(proj_name: &str) -> Result<Vec<SequenceData>, Error> {
    
    println!("Getting data from proton_cli...");

    // proton_cli get-playlist-data <proj-name>
    let output = try!(Command::new("proton_cli")
        .arg("get-playlist-data")
        .arg(proj_name)
        .output()
        .map_err(|_| Error::ProtonCli("Failed to run proton_cli process".to_string())));

    println!("Checking data for errors...");
    // Make sure no error thrown
    let err_str = str::from_utf8(&output.stderr).expect("Data not valid UTF-8");
    if err_str.len() != 0 {
        return Err(Error::ProtonCli(err_str.to_string()));
    }
    
    // proton_cli returns the path to a json data file with everything inside
    let data_file_path = str::from_utf8(&output.stdout)
        .expect("Playlist data file path not valid UTF-8")
        .split("PLAYLIST_DATA:::")
        .nth(1)
        .expect("Invalid output format from proton_cli (expected PLAYLIST_DATA:::)")
        .trim();

    // Read data from the data file
    let plist_data_json = utils::file_as_string(data_file_path)?;

    println!("Parse JSON...");
    // get-playlist-data outputs just the JSON playlist data (as of 11/27/2016),
    // so we just grab the output and call it good
    let plist_data: Vec<SequenceData> = try!(
        json::decode(&plist_data_json)
        .map_err(Error::JsonDecode));

    println!("Transposing data...");
    // Transpose data to frame-major order for easier use later
    let transposed_data = plist_data.into_iter()
        .map(|seq_data| {
            println!("num_frames: {}", seq_data.num_frames);
            let transposed_data = match utils::transpose_data(seq_data.data) {
                Ok(data) => data,
                Err(e) => panic!(e),
            };
            SequenceData {
                name: seq_data.name,
                music_file: seq_data.music_file,
                frame_dur_ms: seq_data.frame_dur_ms,
                num_frames: seq_data.num_frames,
                data: transposed_data
            }
        }).collect::<Vec<SequenceData>>();

    Ok(transposed_data)
}

/// Update the local copy of the show's sequence data
pub fn update_data(cfg: &Config, proj_name: &str) -> Result<(), Error> {
    
    // Get new data
    let new_data = try!(get_data(proj_name));

    // Write new data to files for offline use and save to playlist
    let mut plist_items = Vec::new();
    for sequence_data in new_data {
        // Build sequence data path
        let mut seq_output_path = cfg.output_dir.clone();
        seq_output_path.push_str(&sequence_data.name);
        seq_output_path.push_str(&".json");

        // Build music file path if it exists
        let seq_music_path = sequence_data.music_file.clone().map(|music_file| {
            let mut mus_path = cfg.music_dir.clone();
            mus_path.push_str(&music_file);
            mus_path
        });

        // Save sequence data to file
        print!("Encoding sequence data...");
        let data_json = try!(json::encode(&sequence_data).map_err(Error::JsonEncode));
        println!("sequence data encoded");
        try!(File::create(&seq_output_path)
            .and_then(|mut f| f.write(data_json.as_bytes()))
            .map_err(Error::Io));

        // Add to playlist
        let plist_item = try!(PlaylistItem::new(
            Some(seq_output_path),
            seq_music_path,
            None::<u32>));
        
        plist_items.push(Box::new(plist_item));
    }

    // Make playlist object
    let plist = Playlist {
        name: proj_name.to_string(),
        items: plist_items
    };

    println!("Saving playlist...");
    // Write playlist
    try!(plist.write_to_file(cfg));

    Ok(())
}
