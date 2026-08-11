#![allow(dead_code)]

use super::{DomainId, DomainState};

pub struct DomainControlBlock {
    pub id: DomainId,
    pub state: DomainState,
}

impl DomainControlBlock {
    pub const fn new(id: DomainId) -> Self {
        Self {
            id,
            state: DomainState::Ready,
        }
    }
}

pub struct Scheduler {
    current: Option<DomainId>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self { current: None }
    }

    pub fn activate(&mut self, id: DomainId) {
        self.current = Some(id);
    }

    pub const fn current(&self) -> Option<DomainId> {
        self.current
    }
}
