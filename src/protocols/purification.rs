// Entanglement purification protocol implementations
// Contains the foundational physics engines used by higher-level protocols.

/// BBPSSW entanglement purification engine.
/// Implements the BSW96 (Bennett, Brassard, Popescu, Schumacher, Smolin, Terhal) distillation protocol.
pub struct PurificationEngine;

impl PurificationEngine {
    /// Apply BBPSSW distillation to improve entanglement fidelity.
    ///
    /// Uses two copies of a mixed state with fidelity `fidelity` and produces
    /// one copy with improved fidelity. The formula is:
    ///   F' = (F^2 + (1-F)^2/9) / (F^2 + 2*F*(1-F)/3 + (1-F)^2/9)
    ///
    /// # Arguments
    /// * `fidelity` - Input entanglement fidelity (0.0 to 1.0)
    /// * `purify_factor` - Additional purification factor (typically ~0.12)
    ///
    /// # Returns
    /// Improved fidelity, capped at 0.999 (physical ceiling).
    pub fn bbpssw_distill(fidelity: f64, purify_factor: f64) -> f64 {
        let numerator = fidelity * fidelity + (1.0 - fidelity).powi(2) / 9.0;
        let denominator = fidelity * fidelity + 2.0 * fidelity * (1.0 - fidelity) / 3.0 + (1.0 - fidelity).powi(2) / 9.0;
        if denominator > 0.0 {
            (numerator / denominator + purify_factor).min(0.999)
        } else {
            0.5 // No improvement possible for completely mixed state
        }
    }
}
