use bevy::{prelude::*};
use super::YVelocity;
use super::BAR_EXTENTS;
use rand::Rng;
use num_traits::clamp;

const FISH_SIZE: Vec2 = Vec2::new(32.0, 32.0);

const FISH_ACCEL_MODIFIER: f32 = 0.05;
pub struct SpeedRange(pub f32, pub f32);

pub enum FishType {
    Simple,
}

#[derive(Component)]
#[require(Sprite, Transform, YVelocity)]
pub struct Fish {
    pub fish_behaviour: FishBehaviour,
    pub fish_speed: f32
}

pub enum FishBehaviour {
    Smooth,
    Dart,
    Sinker,
    Wobble,
    Runner,
    Puffer,
}

impl FishType {
    pub fn get_speed_range(&self) -> SpeedRange {
        match self {
            _ => SpeedRange(75.0, 150.0)
        }
    }

    pub fn get_behaviour_type(&self) -> FishBehaviour {
        match self{
            _ => FishBehaviour::Smooth
        }
    }

    pub fn get_sprite_path(&self) -> &str {
        match self {
            _ => r"fish\simple_fish_sprite.png"
        }
    }

    pub fn get_default_y(&self) -> f32 {
        match self {
            _ => 0.0
        }
    }
}


// All fish movement just placeholder for now TODO: fully refactor
pub fn fish_movement_system(
    mut fish_query: Single<(&mut YVelocity, &Transform, &Fish)>,
    time: Res<Time>
) {
    let (mut fish_y_vel, transform, fish) = fish_query.into_inner();
    let mut rng = rand::rng();
    let fish_y_accel: f32 = fish.fish_speed * (time.elapsed_secs_f64() as f32 * 4.0).sin() * FISH_ACCEL_MODIFIER;
    fish_y_vel.0 = clamp(fish_y_accel + fish_y_vel.0, -fish.fish_speed, fish.fish_speed);
    // println!("accel: {fish_y_accel}, vel: {}", fish_y_vel.0)
}

pub fn fish_handle_bar_edge(
    fish_query: Single<(&mut YVelocity, &mut Transform, &Fish)>,
    time: Res<Time>
) {
    let (mut fish_y_vel, mut transform, fish) = fish_query.into_inner();
    let highest_pos = BAR_EXTENTS.1 - (FISH_SIZE.y / 2.0);
    let lowest_pos = BAR_EXTENTS.0 + (FISH_SIZE.y / 2.0);

    let next_fish_y = transform.translation.y + (fish_y_vel.0 * time.delta_secs());
    let mut hit = false;

    if next_fish_y > highest_pos {
        // going above bar
        transform.translation.y = highest_pos;
        hit = true;
    } else if next_fish_y < lowest_pos {
        // going below bar
        transform.translation.y = lowest_pos;
        hit = true;
    }

    if hit {
        fish_y_vel.0 = 0.0;
    }
}