use bevy::prelude::{App, Fixed, Plugin, Time};

use config::SharedConfig;
pub use replication::ReplicationData;
pub use sets::ReplicationSet;

pub mod config;
pub mod events;
pub(crate) mod log;
mod replication;
pub mod sets;
pub mod systems;

pub mod plugin;
