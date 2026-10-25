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
pub mod simulation_results;
mod simulator;

fn main() {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level)
        .init();

    let file_path = cli.data_file_path;
    let data = DataLoader::load_from_path(&file_path).unwrap();

    let mut config = SimulationConfig::default();
    config.simulation_steps = cli.steps.unwrap_or(1_000_000);

    let simulator = Simulator::new(data, config).unwrap();
    let simulation_results = simulator.run();

    let normalized = simulation_results.normalized();
    let mut i = 0;
    for result in normalized {
        if i < 100 {
            i += 1;
        } else {
            break;
        }

        println!("Node: {}, Quality: {}", result.0, result.1);
    }
}
