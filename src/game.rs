mod bird;
mod game_manager;
pub mod game_state;
mod obstacle;

use crate::utils::despawn_screen;
use bevy::{prelude::*, window::PrimaryWindow};
use bird::{update_bird, Bird};
use game_manager::GameManager;
use game_state::GameState;
use obstacle::{spawn_obstacles, update_obstacles, Obstacle};
use rand::rng;

pub const PIXEL_RATIO: f32 = 4.0;
pub const FLAP_FORCE: f32 = 500.;
pub const GRAVITY: f32 = 2000.;
pub const VELOCITY_TO_ROTATION: f32 = 15.;
pub const OBSTACLE_AMOUNT: i32 = 5;
pub const OBSTACLE_VERTICAL_OFFSET: f32 = 30.;
pub const OBSTACLE_SPACING: f32 = 60.;
pub const OBSTACLE_HEIGHT: f32 = 144.;
pub const OBSTACLE_WIDTH: f32 = 32.;
pub const OBSTACLE_GAP_SIZE: f32 = 15.;
pub const OBSTACLE_SCROLL_SPEED: f32 = 150.;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), setup_level.run_if(run_once))
        .add_systems(
            Update,
            (update_bird, update_obstacles, open_menu).run_if(in_state(GameState::InGame)),
        );
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pipe_image: Handle<Image> = asset_server.load("pipe.png");
    let window = window_query
        .get_single()
        .expect("Error in setup_level, no primary window found!");
    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimension: Vec2::new(window.width(), window.height()),
    });
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn((
        Sprite {
            image: asset_server.load("bird.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0. },
    ));
    let mut rand = rng();
    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_image);
}

fn open_menu(mut next_state: ResMut<NextState<GameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyO) {
        next_state.set(GameState::MainMenu);
    }
}
