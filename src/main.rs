use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::time::Duration;

mod blob;
use blob::Blob;
mod position;
use position::Position;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0))),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_blobs, show_blobs).chain())
        .run();
}

fn setup(mut commands: Commands) {
    for x in 1..10 {
        commands.spawn((blob::Blob(x.to_string()), position::Position(0.0, 0.0)));
    }
}

fn move_blobs(query: Query<&mut Position, With<Blob>>) {
    for mut p in query {
        p.0 += 1.;
        p.1 += 1.;
    }
}

fn show_blobs(query: Query<(&Blob, &Position)>) {
    for (Blob(blob), Position(x, y)) in query {
        println!("{} {} {}", blob, x, y);
    }
}
