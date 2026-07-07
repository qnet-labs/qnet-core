use crate::api::request::QNetFile;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QNetDiff {
    pub source_file: String,
    pub target_file: String,
    pub version_changed: Option<(String, String)>,
    pub metadata_changed: Option<MetadataDiff>,
    pub nodes_added: Vec<String>,
    pub nodes_removed: Vec<String>,
    pub nodes_modified: Vec<String>,
    pub links_added: Vec<String>,
    pub links_removed: Vec<String>,
    pub links_modified: Vec<String>,
    pub config_changed: Option<ConfigDiff>,
    pub constraints_changed: Option<ConstraintsDiff>,
    pub extensions_changed: Option<bool>,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct MetadataDiff {
    pub name_changed: Option<(String, String)>,
    pub description_changed: Option<(Option<String>, Option<String>)>,
    pub author_changed: Option<(Option<String>, Option<String>)>,
    pub created_at_changed: Option<(Option<String>, Option<String>)>,
}

#[derive(Debug, Clone)]
pub struct ConfigDiff {
    pub alpha_loss_changed: Option<(Option<f64>, Option<f64>)>,
    pub beta_fidelity_decay_changed: Option<(Option<f64>, Option<f64>)>,
    pub gamma_swapping_changed: Option<(Option<f64>, Option<f64>)>,
    pub max_attempts_changed: Option<(Option<u32>, Option<u32>)>,
}

#[derive(Debug, Clone)]
pub struct ConstraintsDiff {
    pub fidelity_target_changed: Option<(Option<f64>, Option<f64>)>,
    pub max_latency_ms_changed: Option<(Option<f64>, Option<f64>)>,
}

impl QNetDiff {
    pub fn new(source: &str, target: &str) -> Self {
        Self {
            source_file: source.to_string(),
            target_file: target.to_string(),
            version_changed: None,
            metadata_changed: None,
            nodes_added: Vec::new(),
            nodes_removed: Vec::new(),
            nodes_modified: Vec::new(),
            links_added: Vec::new(),
            links_removed: Vec::new(),
            links_modified: Vec::new(),
            config_changed: None,
            constraints_changed: None,
            extensions_changed: None,
            summary: String::new(),
         }
      }

    pub fn generate_summary(&self) -> String {
        let mut summary = format!(
              "Diff between '{}' and '{}':\n",
            self.source_file, self.target_file
         );

        if let Some((v1, v2)) = &self.version_changed {
            summary.push_str(&format!("  Version: {} -> {}\n", v1, v2));
          }

        if let Some(metadata) = &self.metadata_changed {
            summary.push_str("  Metadata changes:\n");
            if let Some((n1, n2)) = &metadata.name_changed {
                summary.push_str(&format!("     - name: '{}' -> '{}'\n", n1, n2));
             }
          }

        if !self.nodes_added.is_empty() {
            summary.push_str(&format!(
                  "  Nodes added: {} ({})\n",
                self.nodes_added.len(),
                self.nodes_added.join(", ")
             ));
          }

        if !self.nodes_removed.is_empty() {
            summary.push_str(&format!(
                  "  Nodes removed: {} ({})\n",
                self.nodes_removed.len(),
                self.nodes_removed.join(", ")
             ));
          }

        if !self.nodes_modified.is_empty() {
            summary.push_str(&format!(
                  "  Nodes modified: {} ({})\n",
                self.nodes_modified.len(),
                self.nodes_modified.join(", ")
             ));
          }

        if !self.links_added.is_empty() {
            summary.push_str(&format!(
                  "  Links added: {} ({})\n",
                self.links_added.len(),
                self.links_added.join(", ")
             ));
          }

        if !self.links_removed.is_empty() {
            summary.push_str(&format!(
                  "  Links removed: {} ({})\n",
                self.links_removed.len(),
                self.links_removed.join(", ")
             ));
          }

        if !self.links_modified.is_empty() {
            summary.push_str(&format!(
                  "  Links modified: {} ({})\n",
                self.links_modified.len(),
                self.links_modified.join(", ")
             ));
          }

        if let Some(config) = &self.config_changed {
            summary.push_str("  Config changes:\n");
            if let Some((a1, a2)) = &config.alpha_loss_changed {
                summary.push_str(&format!("     - alpha_loss: {:?} -> {:?}\n", a1, a2));
             }
          }

        if let Some(constraints) = &self.constraints_changed {
            summary.push_str("  Constraints changes:\n");
            if let Some((f1, f2)) = &constraints.fidelity_target_changed {
                summary.push_str(&format!("     - fidelity_target: {:?} -> {:?}\n", f1, f2));
             }
          }

        if self.extensions_changed == Some(true) {
            summary.push_str("  Extensions changed\n");
          }

        summary
     }
}

pub fn diff_qnet_files(
    source_path: &str,
    target_path: &str,
    source: &QNetFile,
    target: &QNetFile,
) -> QNetDiff {
    let mut diff = QNetDiff::new(source_path, target_path);

      // Check version
    if source.version != target.version {
        diff.version_changed = Some((source.version.clone(), target.version.clone()));
     }

      // Compare metadata
    if source.metadata.name != target.metadata.name
         || source.metadata.description != target.metadata.description
         || source.metadata.author != target.metadata.author
         || source.metadata.created_at != target.metadata.created_at
      {
        let metadata_diff = MetadataDiff {
            name_changed: if source.metadata.name != target.metadata.name {
                Some((source.metadata.name.clone(), target.metadata.name.clone()))
              } else {
                None
              },
            description_changed: if source.metadata.description != target.metadata.description {
                Some((source.metadata.description.clone(), target.metadata.description.clone()))
              } else {
                None
              },
            author_changed: if source.metadata.author != target.metadata.author {
                Some((source.metadata.author.clone(), target.metadata.author.clone()))
              } else {
                None
              },
            created_at_changed: if source.metadata.created_at != target.metadata.created_at {
                Some((source.metadata.created_at.clone(), target.metadata.created_at.clone()))
              } else {
                None
              },
          };
        diff.metadata_changed = Some(metadata_diff);
      }

      // Compare nodes
    let source_nodes: HashMap<&str, &crate::api::request::QNetNode> =
        source.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let target_nodes: HashMap<&str, &crate::api::request::QNetNode> =
        target.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

    for (id, node) in &source_nodes {
        match target_nodes.get(id) {
            None => diff.nodes_removed.push(id.to_string()),
            Some(target_node) => {
                if node != target_node {
                    diff.nodes_modified.push(id.to_string());
                 }
              }
          }
      }

    for (id, _) in &target_nodes {
        if !source_nodes.contains_key(id) {
            diff.nodes_added.push(id.to_string());
         }
      }

      // Compare links
    let source_links: HashMap<String, &crate::api::request::QNetLink> =
        source.links.iter().map(|l| (link_key(l), l)).collect();
    let target_links: HashMap<String, &crate::api::request::QNetLink> =
        target.links.iter().map(|l| (link_key(l), l)).collect();

    for (key, link) in &source_links {
        match target_links.get(key) {
            None => diff.links_removed.push(key.clone()),
            Some(target_link) => {
                if link != target_link {
                    diff.links_modified.push(key.clone());
                 }
              }
          }
      }

    for (key, _) in &target_links {
        if !source_links.contains_key(key) {
            diff.links_added.push(key.clone());
         }
      }

      // Compare config
    if source.config != target.config {
        let config_diff = ConfigDiff {
            alpha_loss_changed: if source.config.as_ref().map(|c| c.alpha_loss)
                  != target.config.as_ref().map(|c| c.alpha_loss)
              {
                Some((source.config.as_ref().and_then(|c| c.alpha_loss),
                      target.config.as_ref().and_then(|c| c.alpha_loss)))
              } else {
                None
              },
            beta_fidelity_decay_changed: if source.config.as_ref().map(|c| c.beta_fidelity_decay)
                  != target.config.as_ref().map(|c| c.beta_fidelity_decay)
              {
                Some((source.config.as_ref().and_then(|c| c.beta_fidelity_decay),
                      target.config.as_ref().and_then(|c| c.beta_fidelity_decay)))
              } else {
                None
              },
            gamma_swapping_changed: if source.config.as_ref().map(|c| c.gamma_swapping)
                  != target.config.as_ref().map(|c| c.gamma_swapping)
              {
                Some((source.config.as_ref().and_then(|c| c.gamma_swapping),
                      target.config.as_ref().and_then(|c| c.gamma_swapping)))
              } else {
                None
              },
            max_attempts_changed: if source.config.as_ref().map(|c| c.max_attempts)
                  != target.config.as_ref().map(|c| c.max_attempts)
              {
                Some((source.config.as_ref().and_then(|c| c.max_attempts),
                      target.config.as_ref().and_then(|c| c.max_attempts)))
              } else {
                None
              },
          };
        diff.config_changed = Some(config_diff);
      }

      // Compare constraints
    if source.constraints != target.constraints {
        let constraints_diff = ConstraintsDiff {
            fidelity_target_changed: if source.constraints.as_ref().map(|c| c.fidelity_target)
                  != target.constraints.as_ref().map(|c| c.fidelity_target)
              {
                Some((source.constraints.as_ref().and_then(|c| c.fidelity_target),
                      target.constraints.as_ref().and_then(|c| c.fidelity_target)))
              } else {
                None
              },
            max_latency_ms_changed: if source.constraints.as_ref().map(|c| c.max_latency_ms)
                  != target.constraints.as_ref().map(|c| c.max_latency_ms)
              {
                Some((source.constraints.as_ref().and_then(|c| c.max_latency_ms),
                      target.constraints.as_ref().and_then(|c| c.max_latency_ms)))
              } else {
                None
              },
          };
        diff.constraints_changed = Some(constraints_diff);
      }

      // Compare extensions
    diff.extensions_changed = Some(source.extensions != target.extensions);

    diff.summary = diff.generate_summary();
    diff
}

fn link_key(link: &crate::api::request::QNetLink) -> String {
    format!("{}->{}", link.from, link.to)
}
