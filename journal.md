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

### Jan 6

i didn't work on my game much today, I just tweaked some scaling settings again to make it look better

### Jan 7

I found this cool glitch shader online, and I implemented it for when you hit the notes.
I added the "fade out" effect by tweening a new "slice drop" paramater I added to teh shader, so after the note is hit (or missed) it will dissapear in segments with a glitch effect.

I tried to use canvasgroups, but aparently they work differently with shaders and the UV being in screen space messed with the glitch shader, so I just applied it to each individual sprite.

Last I made a "safe" (theres no UB, but it prevents dumb typo bugs) wrapper for the shader in rust that allows me to tween and set the values without magic strings. I initially did it manually, but it was a pain so I made a macro that I can also use for future shaders.

I'm planning to make a cool background in glsl (shader) script, maybe even one that reacts to the game audio (that would actually be a really cool Idea I'm totally gonna do that If I can)

Tmrw I need to implement the second track and make a chart for the song though, and hopefully I can also fugure out how to make a character and dodge notes by sunday.

### Jan 8

I made the notes be triggered by seperate inputs, and show up on the high and low tracks to distinguish them.

I then spent a lot of time refactoring how the note instances are created in preparation for deserializing chart files. Now the BeatmapLoader node handles instantiating all the notes with it's `load_beatmap_sprites` method.

The NoteManager now has a reference to the BeatmapLoader to get note positions since they are now children of the BeatmapLoader instead of the NoteManager.

After the BeatmapLoader is ready, the NoteManager calls load_beatmap_sprites and adds all the resulting notes to the event queue.
