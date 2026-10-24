use clap::Parser;

use crate::cli::Cli;
use crate::data_loader::DataLoader;
use crate::markov_config::MarkovConfig;
use crate::markov_simulator::MarkovSimulator;

mod cli;
pub mod data;
mod data_loader;
pub mod markov_config;
mod markov_simulator;
pub mod record;

fn main() {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level)
        .init();

    let file_path = cli.data_file_path;

    let data = DataLoader::load_from_path(&file_path).unwrap();

    let mut config = MarkovConfig::default();
    config.simulation_steps = 1_000_000;

    let mut simulator = MarkovSimulator::new(data);
    simulator.set_config(config);

    let simulation_results = simulator.run_simulation();
    simulation_results.print_normalized_results();
}
