use godot::prelude::*;

struct GodotEvolutionary;

mod algorithms;
mod gd_classes;
mod math;

#[gdextension]
unsafe impl ExtensionLibrary for GodotEvolutionary {}
