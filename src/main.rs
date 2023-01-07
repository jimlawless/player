// player - A command-line .mp3 / .wav file and text-to-speech player
// by Jim Lawless - jimbo@radiks.net - jiml.us
// MIT / X11 license
// See full license at https://github.com/jimlawless/player
//
// Syntax:
//   player -file audio-filename
//  or
//   player -say speech-text
//
// This is one of my first Rust programs, so it's very simple.  I hope to flesh it
// out a bit using more formal command-line parsing, adding more features such as
// playlist files, and such.
//
// The program is really just a thin wrapper around Soloud which does all of the heavy-lifting.
//
// A Windows version of the player is available at:
//    https://jiml.us/downloads/player.zip

use std::env;
use std::io::{self, Write};
use soloud::*;

fn main() {
    let args: Vec<String> = env::args().collect();    
    print!("\nplayer - A command-line audio file / speech player by Jim Lawless\n");
    print!("jimbo@radiks.net\n");
    print!("https://github.com/jimlawless/player\n\n");
    io::stdout().flush().unwrap();
    
    if args.len()<3 {
        syntax(); 
        return;
    }
    let cmd=&args[1];
    let arg=&args[2];
    
    match &cmd[..] {
        "-file" => playsoundfile(arg),          
        "-say" => speak(arg),
        _ => syntax(),
    }    
}

fn speak(text: &str) {
    let sl = Soloud::default().unwrap();
    let mut spk = audio::Speech::default();
    let _tmp=spk.set_text(text);
    sl.play(&spk);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }     
}

fn syntax() {
    print!("Syntax:\nplayer -file audio-filename\n\tor\nplayer -say speech-text\n\n");    
    io::stdout().flush().unwrap();
}

fn playsoundfile(filename: &str) {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    print!("Now playing {}\n",filename);
    io::stdout().flush().unwrap();

    let _tmp=wav.load(&std::path::Path::new(filename));    
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    } 
}

