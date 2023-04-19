//--------------------------------------------------------------------
// Description: Simple example of how to use the anyhow crate.
//--------------------------------------------------------------------

use anyhow::{anyhow, Result};
use std::env;
use std::process::exit;

fn handle_file(filename: &str) -> Result<()> {
    println!("INFO: filename: {:?}", filename);

    // If the file does not exist, or is not readable, this method will return
    // a `std::io::Error` error.
    //
    // But since this function returns an anyhow Result, the anyhow crate will
    // automatically convert this into an _anyhow error_ (basically a String).
    let data = std::fs::read_to_string(filename)?;

    println!("INFO: file contents: {:?}", data);

    Ok(())
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Get the 2nd element (aka the first CLI arg). If not present, convert
    // the returned Option None value to a (std::result::Err) error.
    // Then, convert that Err value into an anyhow Result error using
    // map_err() and the call to the magic anyhow!() macro.
    let filename = args.get(1).ok_or("need file").map_err(|e| anyhow!(e))?;

    handle_file(filename)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        exit(1);
    }
}
