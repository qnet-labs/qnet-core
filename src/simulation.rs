use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    AttemptLinkGeneration {
        from: String,
        to: String,
        base_fidelity: f64,
    },
    #[allow(dead_code)] // wired into processing logic — dead_code fires in test builds
    MemoryDecayDropout { node_id: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationEvent {
    pub time: f64,
    pub event_type: EventType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventWrapper(pub SimulationEvent);

impl Eq for EventWrapper {}

impl Ord for EventWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .0
            .time
            .partial_cmp(&self.0.time)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for EventWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct SimulationRuntime {
    pub timeline: BinaryHeap<EventWrapper>,
    pub metrics_failed_swaps: usize,
    pub metrics_dropped_congestion: usize,
    /// Links successfully established in the current time step (from, to, base_fidelity).
    /// Cleared after each iteration so partial successes don't persist.
    pub(crate) link_establishments: Vec<(String, String, f64)>,
}

impl SimulationRuntime {
    pub fn new() -> Self {
        Self {
            timeline: BinaryHeap::new(),
            metrics_failed_swaps: 0,
            metrics_dropped_congestion: 0,
            link_establishments: Vec::new(),
        }
    }

    /// Process ALL events up to (and including) the given deadline timestamp.
    /// Records successful link establishments and drops from memory decay.
    pub fn process_events_up_to(&mut self, deadline_ms: f64) {
        while let Some(EventWrapper(event)) = self.timeline.peek() {
            if event.time > deadline_ms {
                break;
            }
            let EventWrapper(ev) = self.timeline.pop().unwrap();
            match ev.event_type {
                EventType::AttemptLinkGeneration {
                    from,
                    to,
                    base_fidelity,
                } => {
                    // ~10% swap failure rate as noise (independent of timestamp)
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0.0..1.0) < 0.10 {
                        self.metrics_failed_swaps += 1;
                    } else {
                        self.link_establishments.push((from, to, base_fidelity));
                    }
                }
                EventType::MemoryDecayDropout { node_id: _ } => {
                    self.metrics_dropped_congestion += 1;
                }
            }
        }
    }
}
