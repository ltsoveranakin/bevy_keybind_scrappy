# Bevy Keybind Scrappy

## A work-in-progress keybind abstraction for the bevy game engine

### Usage:

To use, first you must define an "action". Actions is anything that maps a given key to an in game event (i.e. jumping,
walk forwards, walk backwards, etc...)

```rust
const DEFAULT_JUMP_KEY_CODE: KeyCode = KeyCode::Space;

define_action!(Jump, DEFAULT_JUMP_KEY_CODE);

fn jump(jump_bind: KeyBinding<Jump>) {
    if jump_bind.just_pressed() {
        println!("JUMP!");
    }
}

fn main() {
    let mut app = App::new();

    app.register_key_binding::<Jump>()
        .add_systems(Update, jump_action)
        .run();
}
```

Actions can also be defined from a JSON file

`file.json`

```json
{
  "Jump": "Space",
  "MoveUp": "W"
}
```

`main.rs`

```rust
parse_json_file!("./file.json");
```

Now you can use the created actions `Jump` and `MoveUp`

```rust
fn jump(jump_bind: KeyBinding<Jump>) {
    if jump_bind.just_pressed() {
        println!("JUMP!");
    }
}

fn move_up(move_up_bind: KeyBinding<MoveUp>) {
    if move_up_bind.pressed() {
        println!("MOVING UP!");
    }
}
```