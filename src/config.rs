#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicalConfig {
    pub baseline_purify_factor: f64,
    pub speed_of_light_in_fiber_km_ms: f64,
}

impl Default for PhysicalConfig {
    fn default() -> Self {
        Self {
            baseline_purify_factor: 0.12,
            speed_of_light_in_fiber_km_ms: 200.0, // ~200,000 km/s in silica fiber
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulationConfig {
    pub total_time_cutoff_ms: f64,
    pub step_resolution_ms: f64,
    pub physical: PhysicalConfig,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            total_time_cutoff_ms: 5000.0,
            step_resolution_ms: 0.1,
            physical: PhysicalConfig::default(),
        }
    }
}
