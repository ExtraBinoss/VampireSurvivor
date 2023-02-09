use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;
use leafwing_input_manager::prelude::ActionState;
use crate::Action;
use crate::Query;
use crate::Res;
use crate::Time;
use crate::MOVE_FORCE;
use crate::Player;
use crate::PlayerPos;

pub fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce, &Transform), With<Player>>,
    time: Res<Time>,
    mut player_pos: ResMut<PlayerPos>,
)
{
    let mut x_axis = player_pos.move_angle.cos();
    let mut y_axis = player_pos.move_angle.sin();
    for (action_state, mut external_force, transform) in &mut query {
        let mut x_force = 0.0;
        let mut y_force = 0.0;
        //let mut rotation_angle = 0.0;
        x_axis = player_pos.move_angle.sin();
        y_axis = player_pos.move_angle.cos();
    
        if action_state.pressed(Action::MoveLeft) {
            //rotation_angle = Quat::from_rotation_z((-move_angle).to_radians());
            //transform.rotation = rotation;
            player_pos.move_angle -= 0.05;            
        }
        if action_state.pressed(Action::MoveRight) {
            //rotation_angle = transform.with_rotation(Quat::from_rotation_z((move_angle).to_radians()));
            player_pos.move_angle += 0.05;
        }
        if action_state.pressed(Action::MoveUp) {
            y_force = y_axis * MOVE_FORCE;
            x_force = x_axis * MOVE_FORCE;
        }
        if action_state.pressed(Action::MoveDown) {
            y_force = y_axis * -MOVE_FORCE;
            x_force = x_axis * -MOVE_FORCE;
        }
        player_pos.move_angle %= 360.0;
        external_force.force = Vec2::new(x_force, y_force) * time.delta_seconds();
        //transform.rotation = rotation_angle;
    }

}