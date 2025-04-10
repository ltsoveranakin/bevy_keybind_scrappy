use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy_keybind_scrappy::prelude::*;

parse_json_file!(".\\tests\\files\\file.json");

fn modify_key(mut move_up_bind: ModifyBinding<MoveUp>) {
    move_up_bind.set_key(Some(KeyCode::ArrowUp));
}

fn flag_inputs(mut key_input: ResMut<ButtonInput<KeyCode>>) {
    // Flag for move up test
    key_input.press(KeyCode::ArrowUp);
    key_input.release(KeyCode::ArrowUp);
}

fn test_modified_key(move_up_bind: KeyBinding<MoveUp>) {
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
        .add_systems(Update, (modify_key, flag_inputs,test_modified_key).chain())
        .run();
}