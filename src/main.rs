use std::fs::File;

use args::Cli;
use billiards::Table;

use anyhow::{Ok, Result};

use clap::Parser;

mod args;
mod billiards;
mod io;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut file = File::create(args.simulation_output_path)?;

    let mut billiards = Table::new(
        args.fixed_ball_spacing,
        args.white_offset,
        args.initial_velocity,
    );

    io::output_snapshot(&mut file, &billiards)?;

    let mut event_times_list = Vec::new();

    let mut iteration = 0;
    while iteration < args.max_iterations {
        let event_time = billiards.handle_event();

        match event_time {
            Some(time) => {
                event_times_list.push(time);
                io::output_snapshot(&mut file, &billiards)?;
                iteration += 1;
            }
            None => break,
        }
    }

    io::output_event_times(&args.times_output_path, &event_times_list)?;

    Ok(())
}
