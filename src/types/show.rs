use rustc_serialize::json;

use dmx_output::DmxOutput;
use error::Error;
use types::{Config, Playlist, Runnable};
use utils;
use chan::Receiver;
use chan_signal::Signal;

pub type Show<D> = (Receiver<Signal>, Vec<Box<Runnable<D>>>);

pub fn read_playlist(cfg: &Config, proj_name: &str) -> Result<Playlist, Error> {
    println!("Reading playlist");
    let plist_path = Playlist::get_path(cfg, proj_name);
    let plist_json = try!(utils::file_as_string(&plist_path));
    json::decode(&plist_json).map_err(Error::JsonDecode)
}

pub fn load_playlist_items<D>(mut playlist: Playlist) -> Result<Vec<Box<Runnable<D>>>, Error> where D: DmxOutput {
    println!("Loading the playlist items");
    playlist.items.iter_mut()
        .map(|item| item.to_runnable())
        .collect()
}

// pub fn run(&mut self, dmx: &mut D) -> Result<(), Error> { for show_item in &mut self.playlist {
//         let _ = try!(show_item.run(dmx, &self.sigint));
//     }
//     Ok(())
// }
