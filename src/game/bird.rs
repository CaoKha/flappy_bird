use super::{
    game_manager::GameManager,
    obstacle::{spawn_obstacles, Obstacle},
};
use super::{
    FLAP_FORCE, GRAVITY, OBSTACLE_HEIGHT, OBSTACLE_WIDTH, PIXEL_RATIO, VELOCITY_TO_ROTATION,
};
use bevy::prelude::*;
use rand::rng;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

pub fn update_bird(
    mut commands: Commands,
    mut bird_query: Query<(&mut Bird, &mut Transform, Entity), Without<Obstacle>>,
    mut obstacle_query: Query<(&Transform, Entity), With<Obstacle>>,
    mut time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
) {
    if let Ok((mut bird, mut transform, _)) = bird_query.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }
        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += bird.velocity * time.delta_secs();
        transform.translation.y = f32::clamp(
            transform.translation.y,
            -game_manager.window_dimension.y / 2. + 4. * PIXEL_RATIO,
            game_manager.window_dimension.y / 2. - 4. * PIXEL_RATIO,
        );
        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION, -90., 90.).to_radians(),
        );

        let mut dead = false;
        if transform.translation.y <= -game_manager.window_dimension.y / 2. {
            dead = true;
        } else {
            for (pipe_transform, _entity) in obstacle_query.iter() {
                if (pipe_transform.translation.y - transform.translation.y).abs()
                    < OBSTACLE_HEIGHT * PIXEL_RATIO / 2.
                    && (pipe_transform.translation.x - transform.translation.x).abs()
                        < OBSTACLE_WIDTH * PIXEL_RATIO / 2.
                {
                    dead = true;
                    break;
                }
            }
        }

        if dead {
            transform.translation = Vec3::ZERO;
            bird.velocity = 0.;
            // for (_, _, entity) in bird_query.iter_mut() {
            //     commands.entity(entity).despawn();
            // }
            // let mut rand = rng();
            // spawn_obstacles(
            //     &mut commands,
            //     &mut rand,
            //     game_manager.window_dimension.x,
            //     &game_manager.pipe_image,
            // );
        }
    }
}
