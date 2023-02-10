use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::Res;
use crate::PlayerPos;
use crate::Player;
use crate::SPRITE_SCALE;
use crate::PLAYER_SIZE;
use crate::Laser;
use crate::Transform;
use crate::Query;
use crate::AssetServer;

pub fn shooting_lasers (
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    keyboard: Res<Input<KeyCode>>,
    player_pos: ResMut<PlayerPos>,
) {
    let direction = Vec2::new(player_pos.move_angle.sin(), player_pos.move_angle.cos());
    let speed = 1350.0;
    if keyboard.just_pressed(KeyCode::Space) {
            let (x, y) = (player_pos.x, player_pos.y);
            let x_offset = PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE / 2.0;
            
            let mut spawn_laser = |x_offset: f32| {
                commands.spawn(SpriteBundle {
                    texture: asset_server.load("laser_b_01.png"),
                    //transform: Transform::from_translation(Vec3::new(x + x_offset, y + 15.0, 1.0)),
                    transform: Transform::from_translation(Vec3::new(x + player_pos.move_angle.sin(), y + player_pos.move_angle.cos(), 1.0))
                    .with_rotation(Quat::from_rotation_z(-player_pos.move_angle)),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                //.insert(Collider::ball(10.0))
                .insert(Velocity::linear(direction * speed))
                .insert(Laser);
            };
            spawn_laser(x_offset - 10.0);    
    }   
}
