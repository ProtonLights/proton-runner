use std::fmt;
use std::fs::File;
use std::io::Write;

use rustc_serialize::json;

use error::Error;
use types::{Config, PlaylistItem};
use utils;


/// Maps to playlist JSON
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub name: String,
    pub items: Vec<Box<PlaylistItem>>
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, item) in self.items.iter().cloned().enumerate() {
            if item.path.is_some() && item.music.is_some() {
                let path = item.clone().path.unwrap();
                let music = item.music.unwrap();
                try!(write!(f, "({}) Sequence: ({}, {})\n", i, path, music))
            } else if item.path.is_some() {
                try!(write!(f, "({}) Pattern: {}\n", i, item.path.unwrap()))
            } else if item.music.is_some() {
                try!(write!(f, "({}) Music: {}\n", i, item.music.unwrap()))
            } else if item.duration.is_some() {
                try!(write!(f, "({}) Delay: {}\n", i, item.duration.unwrap()))
            } else {
                try!(write!(f, "Invalid playlist item\n"))
            }
        }
        Ok(())
    }
}

impl Playlist {
    pub fn write_to_file(&self, cfg: &Config) -> Result<(), Error> {
        // Build playlist file path
        let plist_path = Playlist::get_path(cfg, &self.name);

        // Write playlist to file
        let plist_json = try!(json::encode(self).map_err(Error::JsonEncode));
        File::create(&plist_path)
            .and_then(|mut f| {
                let _ = f.write(plist_json.as_bytes());
                f.flush()
            })
            .and_then(|_| Ok(()))
            .map_err(Error::Io)
    }

    pub fn get_path(cfg: &Config, proj_name: &str) -> String {
        let mut playlist_path = cfg.playlists_dir.clone();
        playlist_path.push_str(proj_name);
        playlist_path.push_str(".json");
        playlist_path
    }

    pub fn get_playlist(cfg: &Config, proj_name: &str) -> Result<Playlist, Error> {
        // Get path to file
        let playlist_path = Playlist::get_path(cfg, proj_name);

        // Load playlist from file
        let plist_json = try!(utils::file_as_string(&playlist_path));

        // Return decoded project
        json::decode(&plist_json).map_err(Error::JsonDecode)
    }

    pub fn insert_item(&mut self, cfg: &Config, idx: usize, item: PlaylistItem) -> Result<(), Error> {
        // Insert item
        if idx > self.items.len() {
            let end = self.items.len();
            // TODO: Make error
            println!("Cannot insert past index=len(items). Changing index {} to {}", idx, end);
            self.items.insert(end, Box::new(item));
        } else {
            self.items.insert(idx, Box::new(item));
        }

        // Write updated playlist to file
        self.write_to_file(cfg)
    }

    pub fn remove_item(&mut self, cfg: &Config, idx: usize) -> Result<(), Error> {
        // Remove item
        if idx >= self.items.len() {
            let end = self.items.len();
            // TODO: Make error
            println!("Cannot remove index past end of list. Changing index {} to {}", idx, end-1);
            let _ = self.items.remove(end - 1);
        } else {
            let _ = self.items.remove(idx);
        }

        // Write updated playlist to file
        self.write_to_file(cfg)
    }
}
