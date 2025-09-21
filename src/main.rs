use bevy::{prelude::*};
use num_traits::clamp;
use rand::Rng;

use crate::ui::{setup_ui, update_progress_bar_ui};

mod fish;
mod ui;

const GAME_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const BAR_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const FISHING_ROD_BAR_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const FISHING_ROD_BAR_OVER_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
// const FISH_COLOR: Color = Color::srgba(0.5, 0.5, 1.0, 0.5);

pub const BAR_EXTENTS: (f32, f32) = (-128.0, 128.0);
const BAR_WIDTH: f32 = 32.0;
const BASE_FISHING_ROD_BAR_SIZE: f32 = 64.0;

const GRAVITY: f32 = -70.0;
const TERMINAL_VELOCITY: f32 = -200.0;
const PLAYER_ACCEL: f32 = 300.0;
const BOUNCE_DAMPENING_FACTOR: f32 = 0.5;
const ZERO_VEL_THRESHOLD: f32 = 3.0;

const FISH_CAPTURE_POINT: Vec2 = Vec2::new(0.0, 0.0);
const INITIAL_FISH_Y: f32 = 0.0;

const OVER_FILL_RATE: f32 = 0.1;
const NOT_OVER_FILL_RATE: f32 = -0.05;

const INITIAL_CATCH_FILL: f32 = 0.3;

#[derive(Component, Default)]
#[require(Sprite, Transform)]
struct Bar;

#[derive(Component)]
struct FishingRodBar;

#[derive(Component, Deref, DerefMut, Default)]
#[require(Transform)]
pub struct YVelocity(f32);

#[derive(Component)]
#[require(YVelocity)]
struct GravityAffected;

#[derive(Event)]
struct OverFishEvent;

#[derive(Component, Deref, DerefMut, Default)]
struct VertSize(f32);

#[derive(Resource, Deref, DerefMut)]
pub struct CatchProgress(f32);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<OverFishEvent>()
        .init_resource::<CatchProgress>()
        .add_systems(Startup, (setup, ui::setup_ui).chain())
        .add_systems(
            FixedUpdate,
            (apply_gravity,
                        apply_player_accel,
                        handle_bar_bounce,
                        fish::fish_movement_system,
                        fish::fish_handle_bar_edge,
                        apply_y_velocity
                    ).chain()
        )
        .add_systems(FixedUpdate, (trigger_over_fish_event, update_catch_progress).chain())
        .add_systems(Update, (set_fishing_bar_color, ui::update_progress_bar_ui))
        .run();
}

fn setup(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
) {
    let bar_size = BAR_EXTENTS.1 - BAR_EXTENTS.0;
    assert!(bar_size > 0.0);

    commands.spawn(Camera2d);

    // Spawn the big fishing bar behind that does not move
    commands.spawn((
        Bar,
        Sprite::from_color(BAR_COLOR, Vec2::new(BAR_WIDTH, bar_size)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: GAME_SCALE,
            ..default()
        }
    ));

    let initial_fishing_bar_y = BAR_EXTENTS.0 + (BASE_FISHING_ROD_BAR_SIZE / 2.0);

    // Spawn fishing bar that player controls
    commands.spawn((
        FishingRodBar,
        GravityAffected,
        Sprite::from_color(FISHING_ROD_BAR_COLOR, Vec2::new(BAR_WIDTH, BASE_FISHING_ROD_BAR_SIZE)),
        Transform {
            translation: Vec3::new(0.0, initial_fishing_bar_y, 1.0),
            scale: GAME_SCALE,
            ..default()
        },
        VertSize(BASE_FISHING_ROD_BAR_SIZE)
    ));

    let mut rng = rand::rng();
    // let fish_type: fish::FishType = rng.random();
    // TODO: randomly pick
    let fish_type = fish::FishType::Simple;
    let fish_speed_range = fish_type.get_speed_range();
    let fish_speed = rng.random_range(fish_speed_range.0..=fish_speed_range.1);
    let sprite_path = fish_type.get_sprite_path();

    
    commands.spawn((
        fish::Fish {
            fish_behaviour: fish_type.get_behaviour_type(),
            fish_speed: fish_speed
        },
        Sprite::from_image(assert_server.load(sprite_path)),
        Transform {
            translation: Vec3::new(0.0, INITIAL_FISH_Y, 2.0),
            scale: GAME_SCALE,
            ..default()
        }
    ));
}

// fn spawn_fish(fish_type: fish::FishType, fish_y: f32) {

// }
impl Default for CatchProgress{
    fn default() -> Self {
        Self(INITIAL_CATCH_FILL)
    }
}

// uses VertSize of box to calculate against fish
fn trigger_over_fish_event(
    bar_query: Single<(&Transform, &VertSize), With<FishingRodBar>>,
    fish_query: Single<&Transform, With<fish::Fish>>,
    mut ev_overfish: EventWriter<OverFishEvent>
) {
    let (bar_transform, bar_vert_size) = bar_query.into_inner();
    let fish_transform = fish_query.into_inner();
    let bar_translation = bar_transform.translation;

    let fish_check_point = fish_transform.translation.y + FISH_CAPTURE_POINT.y;
    if (bar_translation.y - bar_vert_size.0 / 2.0 <= fish_check_point)
        && (fish_check_point <= bar_translation.y + bar_vert_size.0 / 2.0)
        {
            ev_overfish.write(OverFishEvent);
        }
}

fn update_catch_progress(
    mut catch_progress: ResMut<CatchProgress>,
    ev_overfish: EventReader<OverFishEvent>,
    time: Res<Time>
) {
    let is_over = !ev_overfish.is_empty();
    let fill_diff = if is_over {OVER_FILL_RATE * time.delta_secs()}
        else {NOT_OVER_FILL_RATE * time.delta_secs()};
    catch_progress.0 = clamp(catch_progress.0 + fill_diff, 0.0, 1.0);
    // println!("{}", catch_progress.0 * 100.0);
}

fn set_fishing_bar_color(
    bar_query: Single<&mut Sprite, With<FishingRodBar>>,
    ev_overfish: EventReader<OverFishEvent>
) {
    let mut sprite = bar_query.into_inner();
    if ev_overfish.is_empty() {
        sprite.color = FISHING_ROD_BAR_COLOR;
    } else {
        sprite.color = FISHING_ROD_BAR_OVER_COLOR;
    }
}

fn apply_gravity(
    mut gravity_query: Query<&mut YVelocity, With<GravityAffected>>,
    time: Res<Time>
) {
    for mut y_vel in gravity_query.iter_mut() {
        // TODO: why doesn't my deref work?
        if y_vel.0 >= TERMINAL_VELOCITY{
            y_vel.0 = (y_vel.0 + (GRAVITY * time.delta_secs())).max(TERMINAL_VELOCITY);
        }

    }
}


fn apply_player_accel(
    bar_query: Single<&mut YVelocity, With<FishingRodBar>>,
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>
) {
    let mut bar_y_vel = bar_query.into_inner();
    if buttons.pressed(MouseButton::Left) {
        bar_y_vel.0 += PLAYER_ACCEL * time.delta_secs()
    }
}

fn handle_bar_bounce(
    bar_query: Single<(&mut YVelocity, &mut Transform), With<FishingRodBar>>,
    time: Res<Time>
) {
    let (mut bar_y_vel, mut bar_transform) = bar_query.into_inner();
    let highest_pos = BAR_EXTENTS.1 - (BASE_FISHING_ROD_BAR_SIZE / 2.0);
    let lowest_pos = BAR_EXTENTS.0 + (BASE_FISHING_ROD_BAR_SIZE / 2.0);

    let next_bar_y = bar_transform.translation.y + (bar_y_vel.0 * time.delta_secs());
    let mut bounce = false;

    if next_bar_y > highest_pos {
        // going above bar - bounce down
        bar_transform.translation.y = highest_pos;
        bounce = true;
    } else if next_bar_y < lowest_pos {
        // going below bar - bounce up
        bar_transform.translation.y = lowest_pos;
        bounce = true;
    }

    if bounce {
        bar_y_vel.0 *= -1.0 * BOUNCE_DAMPENING_FACTOR; 
        if bar_y_vel.0.abs() < ZERO_VEL_THRESHOLD {
            bar_y_vel.0 = 0.0;
        }
    }
}

fn apply_y_velocity(
    mut velocity_query: Query<(&YVelocity, &mut Transform)>, 
    time: Res<Time>
) {
    for (y_vel, mut transform) in velocity_query.iter_mut() {
        // println!("{}", y_vel.0);
        transform.translation.y += y_vel.0 * time.delta_secs();
    }
}