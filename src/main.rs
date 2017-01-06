// Some commands don't use config
#![allow(unused_variables)]

extern crate docopt;
extern crate proton_runner;
extern crate rustc_serialize;
extern crate sfml;

use std::{env, io};

use docopt::Docopt;

use proton_runner::DmxOutput;
use proton_runner::error::Error;
use proton_runner::types::{Config, Playlist, Show};


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton_runner add-playlist-item <proj-name> <plist-idx> [--seq=<seq-path>] [--music=<music-path>] [--dur=<duration>]
  ./proton_runner allon <dmx-port>
  ./proton_runner alloff <dmx-port>
  ./proton_runner get-playlist <proj-name>
  ./proton_runner set <dmx-chan> (on | off) <dmx-port>
  ./proton_runner rangeon <chan-start> <chan-end> <dmx-port>
  ./proton_runner rangeoff <chan-start> <chan-end> <dmx-port>
  ./proton_runner remove-playlist-item <proj-name> <plist-idx>
  ./proton_runner run-show <proj-name> <dmx-port> [<plist-offset>]
  ./proton_runner update-data <proj-name>
  ./proton_runner (-h | --help)

Options:
  --seq=<seq-path>      Path to playlist item's sequence data file (can be excluded)
  --music=<music-path>  Path to playlist item's music file (in .ogg format, can be excluded)
  --dur=<duration>      Duration of the playlist item in milliseconds
  -h --help             Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_chan_start: Option<u32>,
    arg_chan_end: Option<u32>,
    arg_dmx_chan: Option<u32>,
    arg_dmx_port: Option<String>,
    arg_plist_idx: Option<u32>,
    arg_plist_offset: Option<u32>,
    arg_proj_name: Option<String>,
    cmd_on: bool,
    cmd_off: bool,
    flag_dur: Option<u32>,
    flag_music: Option<String>,
    flag_seq: Option<String>,
}

fn main() {
    // Get command line arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // Below unwrap()'s are safe within Docopt's usage rules
    // Match command to function to run
    let command: fn(Args, Config) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
        "add-playlist-item" => run_add_playlist_item,
        "allon" => run_all_on,
        "alloff" => run_all_off,
        "get-playlist" => run_get_playlist,
        "set" => run_set,
        "rangeon" => run_range_on,
        "rangeoff" => run_range_off,
        "remove-playlist-item" => run_remove_playlist_item,
        "run-show" => run_run_show,
        "update-data" => run_update_data,
        _ => panic!("Invalid first argument"),
    };

    // Read config file
    let config = match Config::new("config.yaml") {
        Ok(cfg) => cfg,
        Err(e) => panic!("{:?}", e.to_string()),
    };

    // Run command's function
    let result = command(args, config);
    match result {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e.to_string()),
    };
}

fn run_add_playlist_item(args: Args, cfg: Config) -> Result<(), Error> {
    let proj_name = args.arg_proj_name.unwrap();
    let plist_idx = args.arg_plist_idx.unwrap();
    let seq_path = args.flag_seq;
    let music_path = args.flag_music;
    let duration = args.flag_dur;

    proton_runner::playlist::add_item(&cfg, &proj_name, plist_idx, seq_path, music_path, duration)
}

fn run_all_on(args: Args, cfg: Config) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    
    proton_runner::commands::all_on(&mut dmx)
}

fn run_all_off(args: Args, cfg: Config) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    
    proton_runner::commands::all_off(&mut dmx)
}

fn run_get_playlist(args: Args, cfg: Config) -> Result<(), Error> {
    let proj_name = args.arg_proj_name.unwrap();
    let playlist = try!(Playlist::get_playlist(&cfg, &proj_name));
    Ok(println!("{}", playlist))
}

fn run_range_on(args: Args, cfg: Config) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    let chan_start = args.arg_chan_start.unwrap();
    let chan_end = args.arg_chan_end.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    let start = dmx_bounded(chan_start);
    let end = dmx_bounded(chan_end);

    proton_runner::commands::range_on(&mut dmx, start, end)
}

fn run_range_off(args: Args, cfg: Config) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    let chan_start = args.arg_chan_start.unwrap();
    let chan_end = args.arg_chan_end.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    let start = dmx_bounded(chan_start);
    let end = dmx_bounded(chan_end);

    proton_runner::commands::range_off(&mut dmx, start, end)
}

fn run_remove_playlist_item(args: Args, cfg: Config) -> Result<(), Error> {
    let proj_name = args.arg_proj_name.unwrap();
    let plist_idx = args.arg_plist_idx.unwrap();
    
    proton_runner::playlist::remove_item(&cfg, &proj_name, plist_idx)
}

fn run_run_show(args: Args, cfg: Config) -> Result<(), Error> {
    // Prepare show
    let proj_name = args.arg_proj_name.unwrap();
    let dmx_port = args.arg_dmx_port.unwrap();
    let plist_offset = args.arg_plist_offset.unwrap_or(0);
    let show = try!(Show::new(&cfg, &proj_name, &dmx_port, plist_offset));
    println!("Ready!");

    // Wait for user to run
    let mut input = String::new();
    try!(io::stdin().read_line(&mut input).map_err(Error::Io));
    match input.trim() {
        "run" => show.run(),
        "quit" => Ok(()),
        _ => Ok(println!("Invalid command (must be run or quit)"))
    }
}

fn run_set(args: Args, cfg: Config) -> Result<(), Error> {

    let dmx_port = args.arg_dmx_port.unwrap();
    let dmx_chan = args.arg_dmx_chan.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));

    if args.cmd_on {
        proton_runner::commands::range_on(&mut dmx, dmx_chan, dmx_chan)
    } else if args.cmd_off {
        proton_runner::commands::range_off(&mut dmx, dmx_chan, dmx_chan)
    } else {
        Ok(println!("This *should* never happen"))
    }
}

fn run_update_data(args: Args, cfg: Config) -> Result<(), Error> {
    let proj_name = args.arg_proj_name.unwrap();
    proton_runner::data::update_data(&cfg, &proj_name)
}

/// Bind value to range [1, 512]
fn dmx_bounded(unbounded: u32) -> u32 {
    if unbounded < 1 { 1 }
    else if unbounded > 512 { 512 }
    else { unbounded }
}
