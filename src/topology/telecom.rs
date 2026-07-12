use crate::api::request::{LinkDefinition, LinkType, NetworkTopologyPayload, NodeDefinition};

pub fn build() -> NetworkTopologyPayload {
    let mut nodes = Vec::new();
    let mut links = Vec::new();

    // Nodes
    nodes.push(NodeDefinition {
        id: "A".to_string(),
        memory_lifetime_t2: 120.0, // edge
    });
    nodes.push(NodeDefinition {
        id: "B".to_string(),
        memory_lifetime_t2: 150.0, // metro
    });
    nodes.push(NodeDefinition {
        id: "C".to_string(),
        memory_lifetime_t2: 200.0, // regional
    });
    nodes.push(NodeDefinition {
        id: "D".to_string(),
        memory_lifetime_t2: 200.0, // backbone
    });
    nodes.push(NodeDefinition {
        id: "E".to_string(),
        memory_lifetime_t2: 250.0, // gateway
    });
    nodes.push(NodeDefinition {
        id: "F".to_string(),
        memory_lifetime_t2: 180.0, // alternate
    });

    // Links - all fiber for telecom backbone
    links.push(LinkDefinition {
        from_node: "A".to_string(),
        to: "B".to_string(),
        distance_km: 5.0,
        base_fidelity: 0.97,
        generation_rate_hz: 2000.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });
    links.push(LinkDefinition {
        from_node: "B".to_string(),
        to: "C".to_string(),
        distance_km: 80.0,
        base_fidelity: 0.93,
        generation_rate_hz: 1200.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });
    links.push(LinkDefinition {
        from_node: "C".to_string(),
        to: "D".to_string(),
        distance_km: 600.0,
        base_fidelity: 0.88,
        generation_rate_hz: 600.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });
    links.push(LinkDefinition {
        from_node: "D".to_string(),
        to: "E".to_string(),
        distance_km: 5500.0,
        base_fidelity: 0.75,
        generation_rate_hz: 150.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });

    links.push(LinkDefinition {
        from_node: "C".to_string(),
        to: "F".to_string(),
        distance_km: 900.0,
        base_fidelity: 0.91,
        generation_rate_hz: 500.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });
    links.push(LinkDefinition {
        from_node: "F".to_string(),
        to: "E".to_string(),
        distance_km: 4800.0,
        base_fidelity: 0.80,
        generation_rate_hz: 200.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });

    links.push(LinkDefinition {
        from_node: "B".to_string(),
        to: "F".to_string(),
        distance_km: 1200.0,
        base_fidelity: 0.85,
        generation_rate_hz: 400.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });

    NetworkTopologyPayload { nodes, links }
}
