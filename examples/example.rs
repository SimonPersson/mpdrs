extern crate mpd;

use mpd::{Client, Query};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c = Client::new(TcpStream::connect("127.0.0.1:6600").unwrap()).unwrap();
    let now_playing = c.currentsong()?;
    if let Some(song) = now_playing {
        println!("Metadata:");
        for row in c.readcomments(song)? {
            if let Ok((k, v)) = row {
                println!("{}: {}", k, v);
            }
        }
    } else {
        println!("No song playing.");
    }
    Ok(())
}
