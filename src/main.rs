use args::Cli;
use billiards::Table;

use clap::Parser;

mod args;
mod billiards;

fn main() {
    let args = Cli::parse();

    let mut billiards = Table::new(
        args.fixed_ball_spacing,
        args.white_offset,
        args.initial_velocity,
    );

    while billiards.handle_event() {}
}
