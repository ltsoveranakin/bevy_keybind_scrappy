use bevy::prelude::*;
use bevy_keybind_scrappy::prelude::*;

const JUMP_KEY_CODE: KeyCode = KeyCode::Space;
const MOVE_UP_KEY_CODE: KeyCode = KeyCode::ArrowUp;

define_action!(Jump, JUMP_KEY_CODE);
define_action!(MoveUp, MOVE_UP_KEY_CODE);

fn jump_action(jump_bind: KeyBinding<Jump>) {
    if jump_bind.just_pressed() {
        println!("JUMP!");
    }
}

fn move_up_action(move_up_bind: KeyBinding<MoveUp>) {
    if move_up_bind.pressed() {
        println!("Move upwards!!")
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .register_key_binding::<Jump>()
        .register_key_binding::<MoveUp>()
        .add_systems(Update, (jump_action, move_up_action))
        .run();
}
