use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Multi Agent Simulation", author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub fixed_ball_spacing: bool,
    #[arg(short, long, default_value_t = 0.0)]
    pub white_offset: f64,
    #[arg(short, long, default_value_t = 2.0)]
    pub initial_velocity: f64,
    #[arg(short, long, default_value_t = String::from("./output.xyz"))]
    pub simulation_output_path: String,
    #[arg(short, long)]
    pub graph_path: Option<String>,
}