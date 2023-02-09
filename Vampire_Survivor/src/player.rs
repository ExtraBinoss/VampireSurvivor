use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::Res;
use crate::PlayerPos;
use crate::Player;
use crate::SPRITE_SCALE;
use crate::PLAYER_SIZE;
use crate::Time;
use crate::Laser;
use crate::Transform;
use crate::VelocityLaser;
use crate::Query;
use crate::AssetServer;
//use crate::LaserPos;

pub fn shooting_lasers (
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    //mut laser_entity: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
    asset_server: Res<AssetServer>,
    //time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    //laser_velocity: ResMut<VelocityLaser>,
    player_pos: ResMut<PlayerPos>,
    //mut laser_pos: ResMut<LaserPos>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let (x, y) = (player_pos.x, player_pos.y);
        let x_offset = PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE / 2.0;
        
        let mut spawn_laser = |x_offset: f32| {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("laser_b_01.png"),
                transform: Transform::from_translation(Vec3::new(x + x_offset, y + 15.0, 1.0)),
                ..default()
            })
        .insert(Laser)
        // .insert(LaserPos {
        //     x: player_pos.x,
        //     y: player_pos.y,
        // })
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 20.0,
            angular_damping: 20000.0,
        })
        .insert(Restitution::coefficient(1.0))
        .insert(VelocityLaser {
            rotation: player_pos.move_angle,
            x: player_pos.move_angle.sin(),
            y: player_pos.move_angle.cos(),
        });
        };
        spawn_laser(x_offset - 10.0);
        //transform.rotate(Quat::from_rotation_z(player_pos.move_angle));
    }
    // for mut transform in &mut query {
    //     if (laser_pos.x > player_pos.x + 2220.0 / 2.0 && laser_pos.x < player_pos.x - 2220.0 / 2.0) && (laser_pos.y > player_pos.y + 1380.0 / 2.0 && laser_pos.x < player_pos.x - 1380.0 / 2.0) {
    //         //commands.entity(laser_entity).despawn();
    //     }
    //     laser_pos.x += laser_velocity.x * time.delta_seconds();
    //     laser_pos.y += laser_velocity.y * time.delta_seconds();
    //     laser_pos.x = transform.translation.x;
    //     laser_pos.y = transform.translation.y;
    // }
}
