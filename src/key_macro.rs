#[macro_export]
macro_rules! define_action {
    ($action_name: ident, $default_key_code: expr) => {
        bevy_keybind_scrappy::prelude::define_action_full!($action_name, Some($default_key_code));
    };

    ($action_name: ident) => {
        bevy_keybind_scrappy::prelude::define_action_full!($action_name, None);
    };
}

#[macro_export]
macro_rules! define_action_full {
    ($action_name: ident, $default_key_code: expr) => {
        #[derive(Reflect)]
        pub struct $action_name;

        impl Action for $action_name {
            fn get_default_key_code() -> Option<KeyCode> {
                $default_key_code
            }

            fn get_action_name() -> &'static str {
                stringify!($action_name)
            }
        }
    };
}

pub use {define_action, define_action_full};
