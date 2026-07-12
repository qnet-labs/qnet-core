use crate::api::request::{
    LinkDefinition, LinkType, NetworkTopologyPayload, NodeDefinition, SatelliteConditions,
};

pub fn build() -> NetworkTopologyPayload {
    let mut nodes = Vec::new();
    let mut links = Vec::new();

    // Ground nodes
    nodes.push(NodeDefinition {
        id: "Toronto".to_string(),
        memory_lifetime_t2: 150.0,
    });
    nodes.push(NodeDefinition {
        id: "Montreal".to_string(),
        memory_lifetime_t2: 180.0,
    });
    nodes.push(NodeDefinition {
        id: "London".to_string(),
        memory_lifetime_t2: 220.0,
    });

    // Satellite node
    nodes.push(NodeDefinition {
        id: "SAT-1".to_string(),
        memory_lifetime_t2: 300.0,
    });

    // Fiber backbone
    links.push(LinkDefinition {
        from_node: "Toronto".to_string(),
        to: "Montreal".to_string(),
        distance_km: 500.0,
        base_fidelity: 0.90,
        generation_rate_hz: 1000.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });
    links.push(LinkDefinition {
        from_node: "Montreal".to_string(),
        to: "London".to_string(),
        distance_km: 5500.0,
        base_fidelity: 0.75,
        generation_rate_hz: 200.0,
        link_type: LinkType::Fiber,
        satellite_conditions: None,
    });

    // Satellite links with realistic conditions
    // Toronto to satellite: clear sky, good visibility
    links.push(LinkDefinition {
        from_node: "Toronto".to_string(),
        to: "SAT-1".to_string(),
        distance_km: 1000.0,
        base_fidelity: 0.98,
        generation_rate_hz: 50.0,
        link_type: LinkType::Satellite,
        satellite_conditions: Some(SatelliteConditions {
            visibility: 0.95,     // 95% clear sky
            weather_factor: 0.98, // minimal weather penalty
        }),
    });
    // SAT-1 to London: moderate cloud cover
    links.push(LinkDefinition {
        from_node: "SAT-1".to_string(),
        to: "London".to_string(),
        distance_km: 6000.0,
        base_fidelity: 0.97,
        generation_rate_hz: 50.0,
        link_type: LinkType::Satellite,
        satellite_conditions: Some(SatelliteConditions {
            visibility: 0.85,     // 85% visibility due to clouds
            weather_factor: 0.90, // 10% weather penalty
        }),
    });

    NetworkTopologyPayload { nodes, links }
}
