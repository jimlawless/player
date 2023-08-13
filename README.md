# player
Command line audio file and text-to-speech player

Syntax:

    player -file audio-filename

or

    player -say speech-text

This is one of my first Rust programs, so it's very simple.  I hope to flesh it out a bit using more formal command-line parsing, adding more features such as playlist files.  The program is really just a thin wrapper around Soloud which does all of the heavy-lifting.

A Windows version of the player is available at: https://jimlawless.net/downloads/player.zip and https://jiml.us/downloads/player.zip
