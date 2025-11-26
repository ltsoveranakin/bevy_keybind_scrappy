pub trait Action: bevy_reflect::Reflect + bevy_reflect::TypePath + Send + Sync + 'static {
    fn get_default_key_code() -> Option<bevy_input::keyboard::KeyCode>;

    fn get_action_name() -> &'static str;
}
