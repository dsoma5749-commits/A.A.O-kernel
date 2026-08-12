use super::ring3::{DomainId, DomainState};
use crate::arch::x86_64::context::switch_context;

pub const MAX_PROCESSES: usize = 16;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ProcessControlBlock {
    pub id: DomainId,
    pub state: DomainState,
    pub rsp: u64,
}

impl ProcessControlBlock {
    pub const fn empty() -> Self {
        Self {
            id: DomainId(0),
            state: DomainState::Terminated,
            rsp: 0,
        }
    }
}

pub struct PreemptiveScheduler {
    pub processes: [ProcessControlBlock; MAX_PROCESSES],
    pub current_index: usize,
    pub process_count: usize,
}

impl PreemptiveScheduler {
    pub const fn new() -> Self {
        Self {
            processes: [ProcessControlBlock::empty(); MAX_PROCESSES],
            current_index: 0,
            process_count: 0,
        }
    }

    /// Register a new process into the scheduler queue
    #[allow(dead_code)]
    pub fn spawn(&mut self, id: u64, stack_pointer: u64) -> Result<usize, &'static str> {
        if self.process_count >= MAX_PROCESSES {
            return Err("Scheduler queue full");
        }

        let idx = self.process_count;
        self.processes[idx] = ProcessControlBlock {
            id: DomainId(id),
            state: DomainState::Ready,
            rsp: stack_pointer,
        };
        self.process_count += 1;
        Ok(idx)
    }

    /// Preemptive Round-Robin Scheduler Triggered on APIC Timer Tick
    pub unsafe fn schedule_next(&mut self) {
        if self.process_count < 2 {
            return; // No switch needed for single task
        }

        let current_idx = self.current_index;
        let mut next_idx = (current_idx + 1) % self.process_count;

        // Find next executable process
        while self.processes[next_idx].state != DomainState::Ready
            && self.processes[next_idx].state != DomainState::Running
        {
            next_idx = (next_idx + 1) % self.process_count;
            if next_idx == current_idx {
                return;
            }
        }

        if current_idx == next_idx {
            return;
        }

        self.processes[current_idx].state = DomainState::Ready;
        self.processes[next_idx].state = DomainState::Running;
        self.current_index = next_idx;

        let old_rsp_ptr = &mut self.processes[current_idx].rsp as *mut u64;
        let new_rsp = self.processes[next_idx].rsp;

        // Perform Assembly Context Switch
        switch_context(old_rsp_ptr, new_rsp);
    }
}

pub static mut SCHEDULER: PreemptiveScheduler = PreemptiveScheduler::new();
