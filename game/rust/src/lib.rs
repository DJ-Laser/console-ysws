use godot::prelude::*;

mod gameplay;
mod rhythm;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
