use types::Show;
use commands;
use error::Error;
use dmx_output::DmxOutput;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use chan::Receiver;
use chan_signal::Signal;

const HISTORY_FILE: &str = "history.txt";

pub fn repl<D>(mut dmx: D, mut show: Show<D>) -> Result<(), Error> where D: DmxOutput {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY_FILE)?;
    rl.set_history_max_len(100);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let tokens: Vec<&str> = line.trim().split_whitespace().collect();
                if !line.is_empty() {
                    rl.add_history_entry(&line);
                    execute(&mut dmx, &mut show, tokens)?;
                }
            },
            Err(ReadlineError::Interrupted) => {
                continue
            }
            Err(_) => {
                // EOF, or any other circumstances
                break
            }
        }
    }
    
    rl.save_history(HISTORY_FILE).unwrap();
    Ok(())
}

fn execute<D: DmxOutput>(dmx: &mut D, show: &mut Show<D>, tokens: Vec<&str>) -> Result<(), Error> {
    match tokens[0] {
        "run" => show.run(dmx),
        "allon" => commands::all_on(dmx),
        "alloff" => commands::all_off(dmx),
        "seton" => seton(dmx, tokens),
        "setoff" => setoff(dmx, tokens),
        "rangeon" => rangeon(dmx, tokens),
        "rangeoff" => rangeoff(dmx, tokens),
        unknown => Ok(println!("Unknown command: {}", unknown))
    }
}

fn print(message: &str) -> Result<(), Error> {
    println!("{}", message);
    Ok(())
}

fn seton<D: DmxOutput>(dmx: &mut D, tokens: Vec<&str>) -> Result<(), Error> {
    if tokens.len() != 2 {
        return print("USAGE: seton <channel>");
    }
    match tokens[1].parse() {
        Ok(chan) => commands::range_on(dmx, chan, chan),
        _ => print("Channel must parse to u32")
    }
}

fn setoff<D: DmxOutput>(dmx: &mut D, tokens: Vec<&str>) -> Result<(), Error> {
    if tokens.len() != 2 {
        return print("USAGE: setoff <channel>");
    }
    match tokens[1].parse() {
        Ok(chan) => commands::range_off(dmx, chan, chan),
        _ => print("Channel must parse to u32")
    }
}

fn rangeon<D: DmxOutput>(dmx: &mut D, tokens: Vec<&str>) -> Result<(), Error> {
    if tokens.len() != 3 {
        return print("USAGE: rangeon <start> <end>")
    }
    match tokens[1].parse() {
        Ok(start) => match tokens[2].parse() {
            Ok(end) => commands::range_on(dmx, start, end),
            _ => print("End must parse to u32")
        },
        _ => print("Start must parse to u32"),
    }
}

fn rangeoff<D: DmxOutput>(dmx: &mut D, tokens: Vec<&str>) -> Result<(), Error> {
    if tokens.len() != 3 {
        return print("USAGE: rangeoff <start> <end>")
    }
    match tokens[1].parse() {
        Ok(start) => match tokens[2].parse() {
            Ok(end) => commands::range_off(dmx, start, end),
            _ => print("End must parse to u32")
        },
        _ => print("Start must parse to u32"),
    }
}
