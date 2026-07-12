use crate::api::request::{QNetFile, QNetLink, QNetNode};
use petgraph::algo::connected_components;
use petgraph::graph::{Graph, NodeIndex};

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub error_type: ValidationErrorKind,
    pub message: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ValidationErrorKind {
    DuplicateNodeId {
        id: String,
    },
    UnknownNodeReference {
        node_id: String,
        link_id: Option<String>,
    },
    SelfLoop {
        link_id: String,
    },
    InvalidDistance {
        link_id: String,
        value: f64,
    },
    InvalidFidelity {
        link_id: String,
        value: f64,
    },
    InvalidGenerationRate {
        link_id: String,
        value: f64,
    },
    GraphDisconnected {
        component_count: usize,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_info = self
            .path
            .as_ref()
            .map(|p| format!(" [{}]", p))
            .unwrap_or_default();

        match &self.error_type {
            ValidationErrorKind::DuplicateNodeId { id } => {
                write!(f, "Duplicate node ID: {}{}", id, path_info)
            }
            ValidationErrorKind::UnknownNodeReference { node_id, link_id } => {
                let link_info = link_id
                    .as_ref()
                    .map(|id| format!(" in link '{}'", id))
                    .unwrap_or_default();
                write!(
                    f,
                    "Unknown node reference '{}'{}{}",
                    node_id, link_info, path_info
                )
            }
            ValidationErrorKind::SelfLoop { link_id } => {
                write!(f, "Self-loop detected in link '{}'", link_id)
            }
            ValidationErrorKind::InvalidDistance { link_id, value } => {
                write!(
                    f,
                    "Invalid distance {} km for link '{}' (must be > 0){}",
                    value, link_id, path_info
                )
            }
            ValidationErrorKind::InvalidFidelity { link_id, value } => {
                write!(
                    f,
                    "Invalid fidelity {} for link '{}' (must be 0 < x <= 1){}",
                    value, link_id, path_info
                )
            }
            ValidationErrorKind::InvalidGenerationRate { link_id, value } => {
                write!(
                    f,
                    "Invalid generation rate {} Hz for link '{}' (must be > 0){}",
                    value, link_id, path_info
                )
            }
            ValidationErrorKind::GraphDisconnected { component_count } => {
                write!(
                    f,
                    "Graph is disconnected with {} components{}",
                    component_count, path_info
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub warning_type: String,
    pub message: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        if self.is_valid {
            report.push_str("Validation passed.\n");
        } else {
            report.push_str("Validation failed.\n");
        }

        if !self.errors.is_empty() {
            report.push_str("\nErrors:\n");
            for error in &self.errors {
                report.push_str(&format!("   - {}\n", error));
            }
        }

        if !self.warnings.is_empty() {
            report.push_str("\nWarnings:\n");
            for warning in &self.warnings {
                report.push_str(&format!(
                    "   - {} [{}]\n",
                    warning.message,
                    warning.path.as_ref().unwrap_or(&"unknown".to_string())
                ));
            }
        }

        report
    }
}

pub struct QNetValidator;

impl QNetValidator {
    pub fn validate_nodes(nodes: &[QNetNode]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let mut seen_ids = std::collections::HashSet::new();

        for node in nodes {
            if !seen_ids.insert(&node.id) {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::DuplicateNodeId {
                        id: node.id.clone(),
                    },
                    message: String::new(),
                    path: None,
                });
            }
        }

        result
    }

    pub fn validate_links(nodes: &[QNetNode], links: &[QNetLink]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let node_ids: std::collections::HashSet<&str> =
            nodes.iter().map(|n| n.id.as_str()).collect();

        for link in links {
            // Check for self-loops
            if link.from == link.to {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::SelfLoop {
                        link_id: link.id.clone().unwrap_or_else(|| "unknown".to_string()),
                    },
                    message: String::new(),
                    path: None,
                });
            }

            // Check node references
            if !node_ids.contains(&link.from.as_str()) {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::UnknownNodeReference {
                        node_id: link.from.clone(),
                        link_id: link.id.clone(),
                    },
                    message: String::new(),
                    path: None,
                });
            }

            if !node_ids.contains(&link.to.as_str()) {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::UnknownNodeReference {
                        node_id: link.to.clone(),
                        link_id: link.id.clone(),
                    },
                    message: String::new(),
                    path: None,
                });
            }

            // Validate distance
            if link.distance_km <= 0.0 {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::InvalidDistance {
                        link_id: link.id.clone().unwrap_or_else(|| "unknown".to_string()),
                        value: link.distance_km,
                    },
                    message: String::new(),
                    path: None,
                });
            }

            // Validate fidelity
            if !(0.0 < link.base_fidelity && link.base_fidelity <= 1.0) {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::InvalidFidelity {
                        link_id: link.id.clone().unwrap_or_else(|| "unknown".to_string()),
                        value: link.base_fidelity,
                    },
                    message: String::new(),
                    path: None,
                });
            }

            // Validate generation rate
            if link.generation_rate_hz <= 0.0 {
                result.add_error(ValidationError {
                    error_type: ValidationErrorKind::InvalidGenerationRate {
                        link_id: link.id.clone().unwrap_or_else(|| "unknown".to_string()),
                        value: link.generation_rate_hz,
                    },
                    message: String::new(),
                    path: None,
                });
            }
        }

        result
    }

    pub fn validate_graph_connected(nodes: &[QNetNode], links: &[QNetLink]) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Build graph
        let mut graph: Graph<(), (), petgraph::Directed> = Graph::new();
        let node_indices: std::collections::HashMap<&str, NodeIndex> = nodes
            .iter()
            .map(|n| (n.id.as_str(), graph.add_node(())))
            .collect();

        for link in links {
            if let (Some(&from_idx), Some(&to_idx)) = (
                node_indices.get(&link.from.as_str()),
                node_indices.get(&link.to.as_str()),
            ) {
                graph.add_edge(from_idx, to_idx, ());
            }
        }

        let components = connected_components(&graph);

        if components > 1 {
            result.add_error(ValidationError {
                error_type: ValidationErrorKind::GraphDisconnected {
                    component_count: components as usize,
                },
                message: String::new(),
                path: None,
            });
        }

        result
    }

    pub fn validate_all(file: &QNetFile) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate nodes
        let node_validation = Self::validate_nodes(&file.nodes);
        for error in node_validation.errors {
            result.add_error(error);
        }

        // Validate links
        let link_validation = Self::validate_links(&file.nodes, &file.links);
        for error in link_validation.errors {
            result.add_error(error);
        }

        // Validate graph connectivity (warning only)
        let connectivity_validation = Self::validate_graph_connected(&file.nodes, &file.links);
        for warning in connectivity_validation.warnings {
            result.add_warning(warning);
        }

        result
    }
}
