use bevy::prelude::*;

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsSound,
    #[default]
    Disabled,
    Paused,
}
