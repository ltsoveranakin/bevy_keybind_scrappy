use bevy::prelude::*;

// pub struct KeybindPlugin;
//
// impl Plugin for KeybindPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(PreUpdate, update_keybinds);
//     }
// }

pub trait Action: Send + Sync + 'static {
    fn get_default_key_code() -> Option<KeyCode>;

    fn get_action_name() -> &'static str;
}
