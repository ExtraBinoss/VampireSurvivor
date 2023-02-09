use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::Res;
use crate::PlayerPos;
use crate::Player;
use crate::SPRITE_SCALE;
use crate::PLAYER_SIZE;
use crate::Laser;
use crate::Transform;
use crate::VelocityLaser;
use crate::Query;
use crate::AssetServer;
use crate::LaserPos;

pub fn shooting_lasers (
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    keyboard: Res<Input<KeyCode>>,
    player_pos: ResMut<PlayerPos>,
    laser_pos: ResMut<LaserPos>,
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
        .insert(VelocityLaser { x: 100., y: 10. });
        };
        spawn_laser(x_offset - 10.0);
    }
}
    //crééer une entité projo
    //créer cette entité lorsqu'on tire
    //faire en sorte que cette entité ait une trajectoire fixe (pour la première arme)
    //l'entié se détruit si elle est hors écran ou si elle percute un objet
    //oritentation du tir
    //on appue sur un bouton pour tirer et y'a un cooldown
    //faire un cooldown de tire qui s'actualise toutes les fois % 0.2
