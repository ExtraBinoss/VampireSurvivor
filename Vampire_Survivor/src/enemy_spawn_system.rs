use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::Res;
use crate::Time;
use crate::ENEMY_MAX;
use crate::Enemy;
use crate::EnemyPos;
use crate::Transform;
use crate::AssetServer;
use crate::SpriteBundle;
use crate::EnemySpawnTimer;

pub fn enemy_spawn_system(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    //mut query : Query<(Entity, &mut Transform), With <Enemy>>,
) {
    let mut enemy_pos = EnemyPos { 
        x: 0.0, 
        y: 0.0,
        health: 100.0,
        speed: 1.0,
        damage: 10.0,
        count: 0.0,
        wave: 0.0,
    };
    //println!("{}", time.elapsed_seconds());
    let mut wave_is_done = true;
    enemy_spawn_timer.0 += time.delta_seconds();
    if wave_is_done == true {
        for _i in 0..ENEMY_MAX {
            if 1.0 <= enemy_spawn_timer.0 {
                enemy_spawn_timer.0 = 0.0;
                commands.spawn(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(enemy_pos.x, enemy_pos.y, 1.0)),
                    texture: asset_server.load("enemy_a_01.png"),
                    ..default()
                }).insert(RigidBody::Dynamic)
                .insert(Collider::ball(42.0))
                .insert(Enemy);
            }
            // wait for spawnRate
            // unity line is : yield return new WaitForSeconds(spawnRate);
        }
        if 4.0 <= enemy_spawn_timer.0 {
            enemy_spawn_timer.0 = 0.0;
            enemy_pos.count += 3.0;
            enemy_pos.wave += 1.0;
            //println!("Wave {} is done!", enemy_pos.wave);
            wave_is_done = false;
        }
        // wait for timeBetweenSpawns
        // unity line is : yield return new WaitForSeconds(timeBetweenWaves);
    }
}