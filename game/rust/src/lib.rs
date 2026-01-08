use godot::prelude::*;

mod gameplay;
mod rhythm;
mod utils;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
