use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InternalNode {
    #[allow(dead_code)] // used as the map key in lookups — field is the identifier
    pub id: String,
    pub t2_lifetime: f64,
}

#[derive(Debug, Clone)]
pub struct InternalLink {
    pub from: String,
    pub to: String,
    pub distance: f64,
    pub base_fidelity: f64,
    pub rate_hz: f64,
    #[allow(dead_code)] // physics model — not wired into scheduler yet
    pub link_type: super::api::request::LinkType,
}

impl InternalLink {
    /// Calculate effective rate based on link type
    #[allow(dead_code)] // physics model — not wired into scheduler yet
    pub fn effective_rate(&self) -> f64 {
        match self.link_type {
            super::api::request::LinkType::Fiber => self.rate_hz,
            super::api::request::LinkType::Satellite => {
                // Satellite links have lower base rates but less distance-dependent loss
                self.rate_hz * 0.5 // Simplified: assume 50% effective rate for satellites
            }
        }
    }

    /// Calculate effective fidelity based on link type physics
    #[allow(dead_code)] // physics model — not wired into scheduler yet
    pub fn effective_fidelity(&self) -> f64 {
        match self.link_type {
            super::api::request::LinkType::Fiber => {
                // Exponential loss for fiber
                let alpha_db_km = 0.2;
                let loss_db = alpha_db_km * self.distance;
                let transmission_efficiency = 10f64.powf(-loss_db / 10.0);
                self.base_fidelity * transmission_efficiency + (1.0 - transmission_efficiency) * 0.5
            }
            super::api::request::LinkType::Satellite => {
                // Lower loss for satellite links
                let alpha_db_km = 0.02;
                let loss_db = alpha_db_km * self.distance;
                let transmission_efficiency = 10f64.powf(-loss_db / 10.0);
                self.base_fidelity * transmission_efficiency + (1.0 - transmission_efficiency) * 0.5
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuantumNetwork {
    pub nodes: HashMap<String, InternalNode>,
    pub links: Vec<InternalLink>,
}

impl QuantumNetwork {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, t2: f64) {
        self.nodes.insert(
            id.to_string(),
            InternalNode {
                id: id.to_string(),
                t2_lifetime: t2,
            },
        );
    }

    pub fn add_link(
        &mut self,
        from: &str,
        to: &str,
        dist: f64,
        fidelity: f64,
        rate: f64,
        link_type: super::api::request::LinkType,
    ) {
        self.links.push(InternalLink {
            from: from.to_string(),
            to: to.to_string(),
            distance: dist,
            base_fidelity: fidelity,
            rate_hz: rate,
            link_type,
        });
    }
}
