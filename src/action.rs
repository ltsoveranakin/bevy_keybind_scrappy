use bevy::prelude::KeyCode;

pub trait Action: Send + Sync + 'static {
    fn get_default_key_code() -> Option<KeyCode>;

    fn get_action_name() -> &'static str;
}
