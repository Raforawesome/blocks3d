use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct PosText;

pub fn setup_pos_text(mut commands: Commands) {
    commands.spawn((
        PosText,
        Text::new("x: 0, y: 0"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..Default::default()
        },
    ));
}

pub fn update_pos_text(
    mut text: Query<&mut Text, With<PosText>>,
    plr_tr: Query<&Transform, With<Player>>,
) {
    let mut text = text.single_mut();
    let plr_tr = plr_tr.single();

    let Vec3 { x, y, z } = plr_tr.translation;
    text.0 = format!("x: {x:.01}, y: {y:.01}, z: {z:.01}");
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pos_text);
        app.add_systems(Update, update_pos_text);
    }
}
