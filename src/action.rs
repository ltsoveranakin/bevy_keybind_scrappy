use bevy::prelude::*;

pub trait Action: Reflect + TypePath + Send + Sync + 'static {
    fn get_default_key_code() -> Option<KeyCode>;

    fn get_action_name() -> &'static str;
}
