use rustc_serialize::json;

use DmxOutput;
use error::Error;
use types::{Config, Playlist, Runnable};
use utils;


pub struct Show {
    playlist: Vec<Box<Runnable>>,
    dmx: DmxOutput
}

impl Show {
    /// Creates a new show starting at playlist item at index offset, 0-indexed
    pub fn new(cfg: &Config, proj_name: &str, dmx_port: &str, offset: u32) -> Result<Show, Error> {
        
        println!("Creating DMX outputter");
        let dmx = try!(DmxOutput::new(dmx_port));

        println!("Reading playlist");
        let plist_path = Playlist::get_path(cfg, proj_name);
        let plist_json = try!(utils::file_as_string(&plist_path));
        let mut plist: Playlist = try!(json::decode(&plist_json).map_err(Error::JsonDecode));

        println!("Prepping the show");
        // Setup playlist items
        let runnable_plist = plist.items.iter_mut()
            .skip(offset as usize)
            .map(|mut plist_item| match plist_item.to_runnable() {
                Ok(r) => r,
                Err(e) => panic!("{}", e)
            })
            .collect::<Vec<Box<Runnable>>>();
        
        let show = Show {
            dmx: dmx,
            playlist: runnable_plist
        };

        Ok(show)
    }

    pub fn run(mut self) -> Result<(), Error> {

        // Run show
        for show_item in self.playlist.into_iter() {
            let _ = try!(show_item.run(&mut self.dmx));
        }

        Ok(())
    }
}
