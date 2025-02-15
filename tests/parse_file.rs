use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy_keybind_scrappy::prelude::*;

parse_json_file!(".\\tests\\files\\file.json");

fn flag_inputs(mut key_input: ResMut<ButtonInput<KeyCode>>) {
    // Flag for jump test
    key_input.press(KeyCode::Space);

    // Flag for move up test
    key_input.press(KeyCode::KeyW);
    key_input.release(KeyCode::KeyW);
}

fn jump_action(jump_bind: KeyBinding<Jump>) {
    assert!(jump_bind.pressed(), "Jump bind should be 'pressed'");
    assert!(
        jump_bind.just_pressed(),
        "Jump bind should have been 'just pressed'"
    );
    assert!(
        !jump_bind.just_released(),
        "Jump bind should not have been 'just released'"
    );
}

fn move_up_action(move_up_bind: KeyBinding<MoveUp>) {
    assert!(!move_up_bind.pressed(), "Move up should not be 'pressed'");
    assert!(
        move_up_bind.just_pressed(),
        "Move up should be 'just pressed'"
    );
    assert!(
        move_up_bind.just_released(),
        "Move up should be 'just released'"
    );
}

#[test]
fn test_press_and_release_keybind() {
    let mut app = App::new();

    app.add_plugins(InputPlugin)
        .register_key_binding::<Jump>()
        .register_key_binding::<MoveUp>()
        .add_systems(Update, (flag_inputs, jump_action, move_up_action).chain())
        .run();
}
