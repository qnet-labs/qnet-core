use crate::api::request::NetworkTopologyPayload;
use crate::topology::{hybrid, repeater_chain, telecom};

#[derive(Clone, Copy)]
pub enum TopologyType {
    TelecomBackbone,
    RepeaterChain { length: usize },
    HybridSatelliteFiber,
}

pub fn generate_topology(t: TopologyType) -> NetworkTopologyPayload {
    match t {
        TopologyType::TelecomBackbone => telecom::build(),
        TopologyType::RepeaterChain { length } => repeater_chain::build(length),
        TopologyType::HybridSatelliteFiber => hybrid::build(),
     }
}
