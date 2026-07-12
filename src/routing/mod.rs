pub mod strategy;

use crate::network::QuantumNetwork;
use strategy::StrategyType;

#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub path: Vec<String>,
    pub composite_fidelity: f64,
}

pub fn find_route(
    network: &QuantumNetwork,
    from: &str,
    to: &str,
    strategy: StrategyType,
) -> Option<RouteInfo> {
    if !network.nodes.contains_key(from) || !network.nodes.contains_key(to) {
        return None;
    }

    // Find all possible paths using DFS
    let mut all_paths: Vec<Vec<(&str, &str)>> = Vec::new();
    let mut stack = vec![(from.to_string(), vec![from.to_string()])];

    while let Some((current, path)) = stack.pop() {
        if current == to {
            let path_edges: Vec<(&str, &str)> = path
                .windows(2)
                .filter_map(|w| {
                    network
                        .links
                        .iter()
                        .find(|l| {
                            (l.from == w[0] && l.to == w[1]) || (l.to == w[0] && l.from == w[1])
                        })
                        .map(|l| {
                            if l.from == w[0] {
                                (l.from.as_str(), l.to.as_str())
                            } else {
                                (l.to.as_str(), l.from.as_str())
                            }
                        })
                })
                .collect();
            // Only add path if all edges exist
            if path_edges.len() == path.len() - 1 {
                all_paths.push(path_edges);
                // eprintln!("Valid path found: {:?}", path);
            }
            continue;
        }

        for link in network.links.iter() {
            let next = if link.from == current {
                &link.to
            } else {
                &link.from
            };
            if next != &current && !path.contains(next) {
                let mut new_path = path.clone();
                new_path.push(next.clone());
                stack.push((next.clone(), new_path));
            }
        }
    }

    if all_paths.is_empty() {
        return None;
    }

    // Select path based on strategy
    let selected = match strategy {
        StrategyType::HighestFidelity => all_paths.iter().max_by(|a, b| {
            let fidelity_a = a
                .iter()
                .map(|(_, e)| {
                    network
                        .links
                        .iter()
                        .find(|l| l.to == **e || l.from == **e)
                        .map(|l| l.base_fidelity)
                        .unwrap_or(0.0)
                })
                .fold(1.0, |acc, f| acc * f);
            let fidelity_b = b
                .iter()
                .map(|(_, e)| {
                    network
                        .links
                        .iter()
                        .find(|l| l.to == **e || l.from == **e)
                        .map(|l| l.base_fidelity)
                        .unwrap_or(0.0)
                })
                .fold(1.0, |acc, f| acc * f);
            fidelity_a
                .partial_cmp(&fidelity_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })?,
        StrategyType::LowestLatency => all_paths.iter().min_by(|a, b| {
            let latency_a = a
                .iter()
                .map(|(_, e)| {
                    network
                        .links
                        .iter()
                        .find(|l| l.to == **e || l.from == **e)
                        .map(|l| l.distance)
                        .unwrap_or(0.0)
                })
                .sum::<f64>();
            let latency_b = b
                .iter()
                .map(|(_, e)| {
                    network
                        .links
                        .iter()
                        .find(|l| l.to == **e || l.from == **e)
                        .map(|l| l.distance)
                        .unwrap_or(0.0)
                })
                .sum::<f64>();
            latency_a
                .partial_cmp(&latency_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })?,
        StrategyType::HighestSuccess => all_paths.iter().max_by(|a, b| {
            let success_a = a
                .iter()
                .map(|(_, e)| {
                    network
                        .links
                        .iter()
                        .find(|l| l.to == **e || l.from == **e)
                        .map(|l| l.base_fidelity * l.rate_hz)
                        .unwrap_or(0.0)
                })
                .sum::<f64>();
            let success_b = b
                .iter()
                .map(|(_, e)| {
                    network
                        .links
                        .iter()
                        .find(|l| l.to == **e || l.from == **e)
                        .map(|l| l.base_fidelity * l.rate_hz)
                        .unwrap_or(0.0)
                })
                .sum::<f64>();
            success_a
                .partial_cmp(&success_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })?,
    };

    // Build path string vector
    let mut path_vec: Vec<String> = vec![from.to_string()];
    let mut composite_fidelity = 1.0;
    for (src, dst) in selected {
        path_vec.push(dst.to_string());
        composite_fidelity *= network
            .links
            .iter()
            .find(|l| (l.from == *src && l.to == *dst) || (l.to == *src && l.from == *dst))
            .map(|l| l.base_fidelity)
            .unwrap_or(0.0);
    }

    Some(RouteInfo {
        path: path_vec,
        composite_fidelity,
    })
}
