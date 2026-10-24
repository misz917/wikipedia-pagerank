use std::collections::HashMap;

use crate::{data::Data, markov_config::MarkovConfig};

#[derive(Debug)]
pub struct Markov {
    data: Data,
    markov_config: MarkovConfig,
}

impl Markov {
    pub fn new(data: Data) -> Self {
        Markov {
            data,
            markov_config: MarkovConfig::default(),
        }
    }

    pub fn set_config(&mut self, markov_config: MarkovConfig) {
        self.markov_config = markov_config;
    }

    pub fn run_simulation(self) -> SimulationResults {
        self.markov_config.test_config().unwrap();
        tracing::info!("Starting simulation");

        let mut simulation_results = SimulationResults::new(&self.data);
        let mut current_node = self.data.get_random().unwrap();
        let mut next_node: u32;
        for step in 1..=self.markov_config.simulation_steps {
            simulation_results.increment(current_node);

            if let Some(connected_nodes) = self.data.get_by_key(current_node) {
                let random_index = rand::random_range(0..connected_nodes.len());
                next_node = connected_nodes[random_index];
            } else {
                next_node = self.data.get_random().unwrap();
            }

            current_node = next_node;
        }

        simulation_results.steps_passed = Some(self.markov_config.simulation_steps);
        tracing::info!("Done. Simulation finished");
        simulation_results
    }
}

#[derive(Debug)]
pub struct SimulationResults {
    pub hashmap: HashMap<u32, u32>,
    pub steps_passed: Option<u64>,
}

impl SimulationResults {
    pub fn new(data: &Data) -> Self {
        let keys = data.0.keys();

        let hashmap: HashMap<u32, u32> = keys.map(|k| (*k, 0)).collect();

        Self {
            hashmap,
            steps_passed: None,
        }
    }

    #[inline]
    pub fn increment(&mut self, key: u32) {
        let value = self.hashmap.get_mut(&key).unwrap();
        *value += 1;
    }

    pub fn print_normalized_results(&self) {
        let keys = self.hashmap.keys();

        let mut quality: f32;
        for k in keys {
            quality = *self.hashmap.get(k).unwrap() as f32 / self.steps_passed.unwrap() as f32;
            println!("Node: {}, Quality: {}", k, quality);
        }
    }
}
