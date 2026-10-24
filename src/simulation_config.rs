use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimulationConfigError {
    #[error("random jump chance should be between 0.0 and 1.0, {0} was provided")]
    RandomJumpChance(f32),
}

#[derive(Debug)]
pub struct SimulationConfig {
    pub random_jump_chance: f32,
    pub simulation_steps: u64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            random_jump_chance: 0.15,
            simulation_steps: 1000,
        }
    }
}

impl SimulationConfig {
    pub fn test_config(&self) -> Result<(), Vec<SimulationConfigError>> {
        let mut errors = Vec::new();

        if self.random_jump_chance > 1.0 || self.random_jump_chance < 0.0 {
            errors.push(SimulationConfigError::RandomJumpChance(
                self.random_jump_chance,
            ));
        }

        tracing::info!("Config test passed");
        Ok(())
    }
}
