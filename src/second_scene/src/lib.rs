use gdnative::*;
use fullscreen_colored_panel::generate_blue as generate_fullscreen_blue_panel;

#[derive(NativeClass)]
#[inherit(Node)]
struct SceneNode;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

#[methods]
impl SceneNode {
    pub fn _init(_owner: Node) -> Self {
        SceneNode
    }

    #[export]
    pub unsafe fn _ready(&self, mut owner: Node) {
        godot_print!("I'm on the second scene !");
        let background = generate_fullscreen_blue_panel(owner);
        owner.add_child(Some(background.to_node()), true);
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<SceneNode>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();