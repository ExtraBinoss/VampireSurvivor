use std::default::Default;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rolling Game".into(),
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
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>)
{
    //2D Camera
    commands.spawn(Camera2dBundle {
        ..default()
    });
    // Spawn Player
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-150.0, 0.0, 1.0)),
        texture: asset_server.load("ball_blue_large.png"),
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
    .insert(Collider::ball(32.0))
    .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
    })
    .insert(Damping {
        linear_damping: 0.6,
        angular_damping: 5.0,
    })
    .insert(Restitution::coefficient(1.0))
    .insert(Player);
}

const MOVE_FORCE: f32 = 1500.0;

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    //let action_state = query.single_mut();
    for (action_state, mut external_force) in &mut query {
        let x_force = 0.0;
        let y_force = 0.0;

        if action_state.just_pressed(Action::MoveLeft)  {
            //x_force -= MOVE_FORCE * time.delta_seconds();
            let x_force = MOVE_FORCE * time.delta_seconds();
            println!("Left");
            println!("ForceLeft: {:?}", x_force);
        }

        if action_state.just_pressed(Action::MoveRight) {
            let x_force = -MOVE_FORCE * time.delta_seconds();
            println!("Right");
            println!("ForceRight: {:?}", x_force);
        }

        if action_state.just_pressed(Action::MoveUp) {
            let y_force = -MOVE_FORCE * time.delta_seconds();
            println!("Up");
            println!("ForceUp: {:?}", y_force);
        }

        if action_state.just_pressed(Action::MoveDown) {
            let y_force =  MOVE_FORCE * time.delta_seconds();
            println!("Down");
            println!("ForceDown: {:?}", y_force);
        }
        external_force.force = Vec2::new(x_force, y_force) * time.delta_seconds();
        println!("Force: {:?}", external_force.force);
    }
}
