use clap::Parser;

use crate::cli::Cli;
use crate::data_loader::DataLoader;
use crate::markov::Markov;
use crate::markov_config::MarkovConfig;

mod cli;
pub mod data;
mod data_loader;
mod markov;
pub mod markov_config;
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

    let mut markov = Markov::new(data);
    markov.set_config(config);

    let simulation_results = markov.run_simulation();
    simulation_results.print_normalized_results();
}
