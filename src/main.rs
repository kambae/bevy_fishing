use bevy::prelude::*;

const GAME_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BAR_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const FISHING_ROD_BAR_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const FISH_COLOR: Color = Color::srgba(0.5, 0.5, 1.0, 0.5);

const BAR_EXTENTS: (f32, f32) = (-128.0, 128.0);
const BAR_WIDTH:f32 = 32.0;
const BASE_FISHING_ROD_BAR_SIZE: f32 = 64.0;

#[derive(Component, Default)]
#[require(Sprite, Transform)]
struct Bar;

#[derive(Component)]
struct FishingRodBar;

#[derive(Component)]
#[require(Bar)]
struct FishBar;

#[derive(Bundle)]
struct FishingRodBarBundle {
    fishing_rod_bar: FishingRodBar,
    transform: Transform,
    sprite: Sprite
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
    commands.spawn(FishingRodBarBundle{
        fishing_rod_bar: FishingRodBar,
        sprite: Sprite::from_color(FISHING_ROD_BAR_COLOR, Vec2::new(BAR_WIDTH, BASE_FISHING_ROD_BAR_SIZE)),
        transform: Transform {
            translation: Vec3::new(0.0, initial_fishing_bar_y, 1.0),
            scale: GAME_SCALE,
            ..default()
        }
    });

}