use bevy::{prelude::*};
use super::CatchProgress;

// const PROGRESS_BAR_LOCATION: Vec2 = Vec2::new(600.0, 0.0);
const PROGRESS_BAR_SIZE: Vec2 = Vec2::new(32.0, 500.0);
const PROGRESS_BAR_BACKGROUND: Color = Color::srgb(100.0, 100.0, 100.0);
const PROGRESS_BAR_PROGRESS: Color = Color::srgb(0.0, 1.0, 0.0);

#[derive(Component)]
pub struct FillBar;

pub fn setup_ui (
    mut commands: Commands,
 ) {
    commands.spawn((
        Node {
            width: Val::Px(PROGRESS_BAR_SIZE.x),
            height: Val::Px(PROGRESS_BAR_SIZE.y),
            position_type: PositionType::Absolute,
            left: Val::Px(1200.0),
            bottom: Val::Px(50.0),
            align_items: AlignItems::End,
            ..Default::default()
        },
        BackgroundColor(PROGRESS_BAR_BACKGROUND)
    ))
    .with_children(|parent| {
        parent.spawn(
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                FillBar,
                BackgroundColor(PROGRESS_BAR_PROGRESS)
            )
        );
    });
}

pub fn update_progress_bar_ui(
    catch_progress: Res<CatchProgress>,
    bar_query: Single<&mut Node, With<FillBar>>
) {
    let mut bar = bar_query.into_inner();
    bar.height = Val::Percent(catch_progress.0 * 100.0)
}