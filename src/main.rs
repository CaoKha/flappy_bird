mod game;
mod menu;

use bevy::{prelude::*, window::ExitCondition};
use game::game_state::GameState;


fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Flappy Bird"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: Vec2::new(512., 512.).into(),
                    ..Default::default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.init_state::<GameState>();
    app.add_systems(Startup, setup)
        .add_plugins(game::game_plugin);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
