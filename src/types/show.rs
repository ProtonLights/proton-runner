use rustc_serialize::json;

use dmx_output::DmxOutput;
use error::Error;
use types::{Config, Playlist, Runnable};
use utils;

// Runnable has to have DmxOutput type as a type parameter in order to
// statically dispatch it, since we store the Runnables as trait objects
// (dynamically dispatched). Therefore Show also parameters on DmxOutput.
pub struct Show<D> where D: DmxOutput {
    playlist: Vec<Box<Runnable<D>>>,
}

// impl Show<D> {
impl <D> Show<D> where D: DmxOutput {
    /// Creates a new show starting at playlist item at index offset, 0-indexed
    pub fn new(cfg: &Config, proj_name: &str, offset: u32) -> Result<Show<D>, Error> {
        println!("Reading playlist");
        let plist_path = Playlist::get_path(cfg, proj_name);
        let plist_json = try!(utils::file_as_string(&plist_path));
        let mut plist: Playlist = try!(json::decode(&plist_json).map_err(Error::JsonDecode));

        println!("Loading the playlist items");
        // Setup playlist items
        let runnable_plist: Vec<Box<Runnable<D>>> = plist.items.iter_mut()
            .skip(offset as usize)
            .map(|mut plist_item| match plist_item.to_runnable() {
                Ok(r) => r,
                Err(e) => panic!("{}", e)
            })
            .collect();
        
        Ok(Show { playlist: runnable_plist })
    }

    /// Run show
    pub fn run(&mut self, dmx: &mut D) -> Result<(), Error> {
        for show_item in &mut self.playlist {
            let _ = try!(show_item.run(dmx));
        }

        Ok(())
    }
}
