use std::time::Instant;

use rand::seq::IndexedRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    data::Data,
    simulation_config::{SimulationConfig, SimulationConfigError},
    simulation_results::SimulationResults,
};

#[derive(Debug)]
pub struct Simulator {
    data: Data,
    config: SimulationConfig,
    source_nodes: Vec<u32>, // for O(1) random jumps
}

impl Simulator {
    pub fn new(data: Data, config: SimulationConfig) -> Result<Self, Vec<SimulationConfigError>> {
        config.test_config()?;
        let source_nodes: Vec<u32> = data.0.keys().copied().collect();

        Ok(Self {
            data,
            config: config,
            source_nodes,
        })
    }

    pub fn run(self) -> SimulationResults {
        let steps = self.config.simulation_steps;
        let threads = rayon::current_num_threads();
        let steps_per_thread = steps / threads as u64;
        let step_division_remainder = steps % threads as u64;

        tracing::info!("Starting simulation");
        tracing::info!(
            threads,
            steps_per_thread,
            step_division_remainder,
            random_jump_chance = self.config.random_jump_chance,
            nodes = self.source_nodes.len()
        );

        let mut combined_results = (0..threads)
            .into_par_iter()
            .map(|thread_num| {
                let steps_to_do = if step_division_remainder > thread_num as u64 {
                    steps_per_thread + 1
                } else {
                    steps_per_thread
                };

                self.run_thread(steps_to_do, thread_num)
            })
            .reduce(
                || SimulationResults::new(&self.data),
                |mut accumulator, other| {
                    accumulator.merge(other);
                    accumulator
                },
            );

        combined_results.nodes = Some(self.source_nodes.len() as u32);
        combined_results.threads = Some(threads);
        combined_results.steps_per_thread = Some(steps_per_thread);
        combined_results.step_division_remainder = Some(step_division_remainder);

        tracing::info!("Done. Simulation finished");
        combined_results
    }

    fn run_thread(&self, steps: u64, thread_num: usize) -> SimulationResults {
        let start = Instant::now();
        let one_percent = (steps / 100).max(1);

        let mut results = SimulationResults::new(&self.data);
        let mut current_node = self.data.get_random().unwrap();

        for i in 0..steps {
            results.record(current_node);
            current_node = self.next_node(current_node);

            if i % one_percent == 0 {
                let progress = i / one_percent;
                tracing::info!("Thread: {:>3}, Progress: {:>3}%", thread_num, progress);
            }
        }

        let elapsed = start.elapsed();
        results.execution_time = Some(elapsed);

        results
    }

    fn next_node(&self, node: u32) -> u32 {
        if rand::random_bool(f64::from(self.config.random_jump_chance)) {
            return self.random_node();
        }

        self.data
            .get_by_key(node)
            .and_then(|edges| edges.choose(&mut rand::rng()))
            .copied()
            .unwrap_or_else(|| self.random_node())
    }

    fn random_node(&self) -> u32 {
        *self
            .source_nodes
            .choose(&mut rand::rng())
            .expect("random_node failed")
    }
}
