use bevy::{prelude::*};

pub struct SpeedRange(f32, f32);

pub enum FishType {
    Simple,
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
            _ => SpeedRange(20.0, 40.0)
        }
    }

    pub fn get_behaviour_type(&self) -> FishBehaviour {
        match self{
            _ => FishBehaviour::Smooth
        }
    }

    pub fn get_sprite_path(&self) -> &str {
        match self {
            _ => "assets/fish/simple_fish_sprite.png"
        }
    }

    pub fn get_default_y(&self) -> 
}

pub fn fish_movement_system(

)