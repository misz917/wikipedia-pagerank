use std::{cmp::Ordering, collections::HashMap};

use crate::data::Data;

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

    pub fn normalized(&self) -> Vec<(u32, f64)> {
        let steps = self.steps.max(1) as f64; // to ensure there won't be division by 0
        let mut scores: Vec<(u32, f64)> = self
            .visits
            .iter()
            .map(|(k, v)| (*k, *v as f64 / steps))
            .collect();

        scores.sort_by(|(ak, av), (bk, bv)| {
            bv.partial_cmp(av)
                .unwrap_or(Ordering::Equal)
                .then(bk.cmp(ak))
        });

        scores
    }
}
