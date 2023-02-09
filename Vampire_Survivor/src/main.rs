use std::default::Default;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::math::*;
use crate::movement::movement;
use crate::enemy_spawn_system::enemy_spawn_system;
use crate::player::shooting_lasers;
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;
const ENEMY1_FORCE: f32 = 8500.0;
const ENEMY_MAX : i32 = 100;
const MOVE_FORCE: f32 = 12500.0;

mod movement;
mod enemy_spawn_system;
mod player;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Laser;

#[derive(Component, Reflect, Resource)]
pub struct LaserPos {
    x: f32,
    y: f32,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

#[derive(Reflect, Resource, Default)]
pub struct EnemySpawnTimer(pub f32);

#[derive(Reflect, Resource)]
pub struct PlayerPos {
    x: f32,
    y: f32,
    move_angle: f32,
    health: f32,
    damage: f32,
    lvl: f32,
    xp: f32,
}

#[derive(Reflect, Resource)]
pub struct EnemyPos {
    x: f32,
    y: f32,
    health: f32,
    speed: f32,
    damage: f32,
    count: f32,
    wave: f32,
}
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct VelocityLaser {
	pub x: f32,
	pub y: f32,
    pub rotation: f32,
}

#[derive(Component, Default)]
struct PositionPlayer(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "LES CANARDS DE L'ESPACE !".into(),
                width: 1920.0,
                height: 1080.0,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(PlayerPos {
            x: 0.0,
            y: 0.0,
            health: 100.0,
            damage: 10.0,
            lvl: 1.0,
            xp: 0.0,
            move_angle: 0.0,
        })
        .insert_resource(EnemyPos {
            x: 0.0,
            y: 0.0,
            health: 100.0,
            speed: 1.0,
            damage: 10.0,
            count: 0.0,
            wave: 0.0,
        })
        .insert_resource(LaserPos {
            x: 0.0,
            y: 0.0,
        })
        .insert_resource(EnemySpawnTimer(0.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(find_player_position)
        .add_system(enemy_spawn_system)
        .add_system(shooting_lasers)
        .run();
}

pub fn find_player_position(
    mut query: Query<(&Transform, &Player)>,
    mut query2: Query<(&Transform, &Laser)>,
    mut cam_transform: Query<&mut Transform, (With<Camera>, Without<Player>, Without<Laser>)>,
    mut player_pos: ResMut<PlayerPos>,
    mut laser_pos: ResMut<LaserPos>,
)
{
    player_pos.x = 0.0;
    player_pos.y = 0.0;
    for (transform, _player) in &mut query {
        player_pos.x = transform.translation.x;
        player_pos.y = transform.translation.y;
        // laser_pos.x = transform.translation.x;
        // laser_pos.y = transform.translation.y;
    }
    for (transform, _laser) in &mut query2 {
        laser_pos.x = transform.translation.x;
        laser_pos.y = transform.translation.y;
        //println!("LASER POS: {}, {}", laser_pos.x, laser_pos.y);
    }
    for mut transform in &mut cam_transform {
        transform.translation.x = player_pos.x;
        transform.translation.y = player_pos.y;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    //2D Camera
    commands.spawn(Camera2dBundle {
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(10.0, 10.0, 0.0)),
        texture: asset_server.load("map.png"),
        ..default()
    });
    // Spawn Player
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-150.0, 0.0, 1.0)),
        texture: asset_server.load("player.png"),
        ..default()
    }).insert(InputManagerBundle::<Action> { 
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Up, Action::MoveUp),
                (KeyCode::Down, Action::MoveDown),
                (KeyCode::Left, Action::MoveLeft),
                (KeyCode::Right, Action::MoveRight),
            ]),
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(40.0))
    .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
    })
    .insert(Damping {
        linear_damping: 2.0,
        angular_damping: 20000.0,
    })
    .insert(Restitution::coefficient(1.0))
    .insert(Player);

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(150.0, 0.0, 1.0)),
        texture: asset_server.load("enemy_a_01.png"),
        ..default()
    }).insert(RigidBody::Dynamic)
    .insert(Collider::ball(32.0))
    .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
    })
    .insert(Damping {
        linear_damping: 20.0,
        angular_damping: 20000.0,
    })
    .insert(Restitution::coefficient(1.0))
    .insert(Enemy);
}


