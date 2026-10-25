use std::collections::HashMap;

use rand::seq::IndexedRandom;

use crate::{
    data::Data,
    simulation_config::{SimulationConfig, SimulationConfigError},
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
        let source_nodes: Vec<u32> = data.0.keys().map(|k| *k).collect();

        Ok(Self {
            data,
            config: config,
            source_nodes,
        })
    }

    pub fn run(self) -> SimulationResults {
        let steps = self.config.simulation_steps;

        tracing::info!("Starting simulation");
        tracing::info!(
            steps,
            random_jump_chance = self.config.random_jump_chance,
            nodes = self.source_nodes.len()
        );

        let mut results = SimulationResults::new(&self.data);
        let mut current_node = self.data.get_random().unwrap();

        for _ in 0..steps {
            results.record(current_node);
            current_node = self.next_node(current_node);
        }

        tracing::info!("simulation finished");
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

#[derive(Debug)]
pub struct SimulationResults {
    pub visits: HashMap<u32, u64>,
    pub steps: u64,
}

impl SimulationResults {
    pub fn new(data: &Data) -> Self {
        let keys = data.0.keys();

        let visits: HashMap<u32, u64> = keys.map(|k| (*k, 0)).collect();

        Self { visits, steps: 0 }
    }

    #[inline]
    pub fn record(&mut self, node: u32) {
        *self.visits.entry(node).or_insert(0) += 1;
        self.steps += 1;
    }

    pub fn print_normalized_results(&self) {
        let keys = self.visits.keys();

        let mut quality: f32;
        for k in keys {
            quality = *self.visits.get(k).unwrap() as f32 / self.steps as f32;
            println!("Node: {}, Quality: {}", k, quality);
        }
    }
}
