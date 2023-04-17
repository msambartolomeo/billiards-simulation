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
    io::output_snapshot(&mut file, &billiards);

    let mut iteration = 0;
    while billiards.handle_event() && iteration < args.max_iterations {
        io::output_snapshot(&mut file, &billiards);
        iteration += 1;
    }

    Ok(())
}
