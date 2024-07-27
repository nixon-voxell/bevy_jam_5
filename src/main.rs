// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_jam_5::AppPlugin;

use clap::Parser;

#[derive(Parser, Debug)]
//#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> AppExit {
    let args = Args::parse();

    App::new()
        .add_plugins(AppPlugin {
            show_debug: args.debug,
        })
        .run()
}
