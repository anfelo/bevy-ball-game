use bevy::prelude::{
    App, IntoSystemConfig, IntoSystemConfigs, IntoSystemSetConfig, Plugin, SystemSet,
};

pub mod components;
mod systems;

use systems::*;

// System Sets are used to group systems together in some
// logical way. This also allows us to define the order of
// execution of different sets
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            // Simple Explicit System Ordering
            // .add_system(player_movement)
            // .add_system(confine_player_movement.after(player_movement))
            // Run one after the other with chain
            // .add_systems((player_movement, confine_player_movement).chain())
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
