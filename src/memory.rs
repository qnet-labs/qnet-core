/// Track qubit memory register capacity during simulation.
#[allow(dead_code)] // not wired into scheduler yet — kept as public API scaffold
pub struct MemoryRegisterTracker {
    pub allocated_qubits: usize,
    pub max_capacity: usize,
}

#[allow(dead_code)]
impl MemoryRegisterTracker {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            allocated_qubits: 0,
            max_capacity,
        }
    }

    #[allow(dead_code)]
    pub fn try_buffer(&mut self) -> bool {
        if self.allocated_qubits < self.max_capacity {
            self.allocated_qubits += 1;
            true
        } else {
            false
        }
    }

    pub fn flush(&mut self) {
        self.allocated_qubits = 0;
    }
}
