use bevy::{prelude::*, window::PrimaryWindow};

pub const PIXEL_RATIO: f32 = 4.0;

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimension: Vec2,
}


