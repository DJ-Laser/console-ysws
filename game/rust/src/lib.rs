use godot::prelude::*;

mod rhythm;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
