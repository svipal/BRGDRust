use godot::engine::{Node, AudioStreamGeneratorPlayback};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Test {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Test {
    #[func]
    fn testPlay(&mut self, mut playback: AudioStreamGeneratorPlayback)  {
        playback.push_frame(Vector2 { x: 0., y: 0. });
    }
}

#[godot_api]
impl NodeVirtual for Test {
        fn init(base: Base<Node>) -> Self {
            Test {
                base
            }
    }
    fn ready(&mut self) {
    }   
}

struct BRTest;

#[gdextension]
unsafe impl ExtensionLibrary for BRTest {}