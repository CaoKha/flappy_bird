use super::game_manager::GameManager;
use super::{
    OBSTACLE_AMOUNT, OBSTACLE_GAP_SIZE, OBSTACLE_HEIGHT, OBSTACLE_SCROLL_SPEED, OBSTACLE_SPACING,
    OBSTACLE_VERTICAL_OFFSET, PIXEL_RATIO,
};
use bevy::prelude::*;
use rand::{rng, rngs::ThreadRng, Rng};

#[derive(Component)]
pub struct Obstacle {
    pipe_direction: f32,
}

fn spawn_obstacle(
    commands: &mut Commands,
    pipe_image: &Handle<Image>,
    translation: Vec3,
    pipe_direction: f32,
) {
    commands.spawn((
        Sprite {
            image: pipe_image.clone(),
            ..Default::default()
        },
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle { pipe_direction },
    ));
}

pub fn spawn_obstacles(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
) {
    for i in 0..OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);
        info!("y_offset: {}", y_offset);
        let x_pos = window_width / 2. + (OBSTACLE_SPACING * PIXEL_RATIO * i as f32);
        spawn_obstacle(
            commands,
            pipe_image,
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position() + y_offset),
            1.,
        );

        spawn_obstacle(
            commands,
            pipe_image,
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position() + y_offset),
            -1.,
        );
    }
}

fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.random_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO
}

fn get_centered_pipe_position() -> f32 {
    info!(
        "center pipe position: {}",
        OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE
    );
    (OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}

pub fn update_obstacles(
    time: Res<Time>,
    game_manager: Res<GameManager>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
) {
    let mut rand = rng();
    let y_offset = generate_offset(&mut rand);
    for (obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * OBSTACLE_SCROLL_SPEED;
        info!("x: {}", transform.translation.x);
        if transform.translation.x + OBSTACLE_GAP_SIZE * PIXEL_RATIO / 2.
            < -game_manager.window_dimension.x / 2.
        {
            transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;
            transform.translation.y =
                -get_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        }
    }
}
