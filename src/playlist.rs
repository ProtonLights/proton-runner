use error::Error;
use types::{Config, Playlist, PlaylistItem};


pub fn remove_item(cfg: &Config, proj_name: &str, plist_idx: u32) -> Result<(), Error> {
    // Get current playlist
    let mut plist = try!(Playlist::get_playlist(cfg, proj_name));

    // Remove item
    plist.remove_item(cfg, plist_idx as usize)
}

pub fn add_item(
    cfg: &Config,
    proj_name: &str,
    plist_idx: u32,
    path: Option<String>,
    music: Option<String>,
    duration: Option<u32>
) -> Result<(), Error> {
    // Get current playlist
    let mut plist = try!(Playlist::get_playlist(cfg, proj_name));

    // Create item to add to playlist
    let plist_item = try!(PlaylistItem::new(path, music, duration));

    // Add to playlist items
    plist.insert_item(cfg, plist_idx as usize, plist_item)
}
