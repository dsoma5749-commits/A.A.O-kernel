#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DomainId(pub u64);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[allow(dead_code)]
pub struct UserEntry {
    pub entry_point: u64,
    pub stack_top: u64,
}

#[allow(dead_code)]
impl UserEntry {
    pub fn new(entry_point: u64, stack_top: u64) -> Self {
        Self {
            entry_point,
            stack_top,
        }
    }
}

#[allow(dead_code)]
pub struct Ring3Domain {
    pub id: DomainId,
    pub state: DomainState,
    pub entry: UserEntry,
}

#[allow(dead_code)]
impl Ring3Domain {
    pub fn new(entry: UserEntry) -> Result<Self, &'static str> {
        Ok(Self {
            id: DomainId(1),
            state: DomainState::Ready,
            entry,
        })
    }
}
