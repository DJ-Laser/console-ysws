# Godot rhythm game thing

### Jan 3

I set up the godot project with rust, then did A LOT of research on how to use the rust bindings and how to make a rhythm game in godot. I found the godot wiki page, and it seems like I can sync the inputs to the music but the patch for syncing hitsounds to the beat isn't merged yet unfortunately.

Tutorials I used: https://www.youtube.com/watch?v=JPBmrUsSeok, https://www.youtube.com/watch?v=_FRiPPbJsFQ

I later found the rhythm game demo project which after fixing the formatting so it would run provided a great foundation for translating the code into rust.

### Jan 4

I did art for the notes (basic shapes in svgs was the best i could do lol) and programmed their functionality in Rust.

They move along the track now, and you can hit them, which will print out your timing in the debug output.

The system if modular and extendable to new note types, but that also means it's a bit harder to create the specific functionality of each note, I think it's worth the complexity though.

This took a long time and im tired now :/

### Jan 5

I tweaked the scrolling behavior and made hold notes propperly miss on an early release.
