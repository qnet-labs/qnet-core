use crate::api::request::{LinkDefinition, LinkType, NetworkTopologyPayload, NodeDefinition};

pub fn build(length: usize) -> NetworkTopologyPayload {
    let mut nodes = Vec::new();
    let mut links = Vec::new();

    for i in 0..length {
        nodes.push(NodeDefinition {
            id: format!("N{}", i),
            memory_lifetime_t2: 150.0,
        });
    }

    for i in 0..(length - 1) {
        links.push(LinkDefinition {
            from_node: format!("N{}", i),
            to: format!("N{}", i + 1),
            distance_km: 100.0,
            base_fidelity: 0.92,
            generation_rate_hz: 800.0,
            link_type: LinkType::Fiber,
            satellite_conditions: None,
        });
    }

    NetworkTopologyPayload { nodes, links }
}
