use crate::keybind_plugin::Action;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::marker::PhantomData;

pub trait RegisterKeyBinding {
    fn register_key_binding<A>(&mut self) -> &mut Self
    where
        A: Action;
}

impl RegisterKeyBinding for App {
    fn register_key_binding<A>(&mut self) -> &mut Self
    where
        A: Action,
    {
        self.insert_resource(KeyBind {
            key_code: A::get_default_key_code(),
            _marker: PhantomData::<A>::default(),
        })
    }
}

/// The underlying KeyBind [`Resource`], which stores the keycode associated with the bind

#[derive(Resource)]
pub(crate) struct KeyBind<A>
where
    A: Action,
{
    pub(crate) key_code: Option<KeyCode>,
    pub(crate) _marker: PhantomData<A>,
}

#[derive(SystemParam)]
pub struct KeyBinding<'w, A>
where
    A: Action,
{
    input: Res<'w, ButtonInput<KeyCode>>,
    key: Res<'w, KeyBind<A>>,
}

impl<'w, A> KeyBinding<'w, A>
where
    A: Action,
{
    pub fn just_pressed(&self) -> bool {
        self.key
            .key_code
            .map_or(false, |code| self.input.just_pressed(code))
    }

    pub fn pressed(&self) -> bool {
        self.key
            .key_code
            .map_or(false, |code| self.input.pressed(code))
    }

    pub fn just_released(&self) -> bool {
        self.key
            .key_code
            .map_or(false, |code| self.input.just_released(code))
    }
}
