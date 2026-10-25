use std::{cmp::Ordering, collections::HashMap, time::Duration};

use crate::data::Data;

#[derive(Debug)]
pub struct SimulationResults {
    pub visits: HashMap<u32, u64>,
    pub nodes: Option<u32>,
    pub threads: Option<usize>,
    pub total_steps: u64,
    pub steps_per_thread: Option<u64>,
    pub step_division_remainder: Option<u64>,
    pub execution_time: Option<Duration>,
}

impl SimulationResults {
    pub fn new(data: &Data) -> Self {
        let keys = data.0.keys();

        let visits: HashMap<u32, u64> = keys.map(|k| (*k, 0)).collect();

        Self {
            visits,
            nodes: None,
            threads: None,
            total_steps: 0,
            steps_per_thread: None,
            step_division_remainder: None,
            execution_time: None,
        }
    }

    #[inline]
    pub fn record(&mut self, node: u32) {
        *self.visits.entry(node).or_insert(0) += 1;
        self.total_steps += 1;
    }

    pub fn normalized(&self) -> Vec<(u32, f64)> {
        let total_steps = self.total_steps.max(1) as f64; // to ensure there won't be division by 0
        let mut scores: Vec<(u32, f64)> = self
            .visits
            .iter()
            .map(|(k, v)| (*k, *v as f64 / total_steps))
            .collect();

        scores.sort_by(|(ak, av), (bk, bv)| {
            bv.partial_cmp(av)
                .unwrap_or(Ordering::Equal)
                .then(bk.cmp(ak))
        });

        scores
    }

    pub fn merge(&mut self, other: SimulationResults) {
        for (node, count) in other.visits {
            *self.visits.entry(node).or_insert(0) += count;
        }
        self.total_steps += other.total_steps;

        self.execution_time = self.execution_time.max(other.execution_time);
    }
}
