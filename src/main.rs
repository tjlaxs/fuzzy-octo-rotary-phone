use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0))),
        )
        .add_systems(Update, update)
        .run();
}

fn update() {
    println!(".");
}
