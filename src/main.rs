use clap::Parser;

use crate::cli::Cli;
use crate::data_loader::DataLoader;
use crate::simulation_config::SimulationConfig;
use crate::simulator::Simulator;

mod cli;
pub mod data;
mod data_loader;
pub mod record;
pub mod simulation_config;
mod simulator;

fn main() {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level)
        .init();

    let file_path = cli.data_file_path;
    let data = DataLoader::load_from_path(&file_path).unwrap();

    let mut config = SimulationConfig::default();
    config.simulation_steps = 1_000_000;

    let simulator = Simulator::new(data, config).unwrap();
    let simulation_results = simulator.run();
    simulation_results.print_normalized_results();
}
