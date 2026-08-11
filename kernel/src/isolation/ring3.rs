#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Ring3DomainId(u64);

impl Ring3DomainId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserEntry {
    pub entry: u64,
    pub stack_top: u64,
}

impl UserEntry {
    pub const fn new(entry: u64, stack_top: u64) -> Self {
        Self { entry, stack_top }
    }

    pub const fn is_valid(self) -> bool {
        self.entry != 0 && self.stack_top > self.entry && (self.stack_top - self.entry) >= 4096
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ring3State {
    Created,
    Ready,
    Running,
    Blocked,
    Stopped,
    Crashed,
}

pub struct Ring3Domain {
    id: Ring3DomainId,
    entry: UserEntry,
    state: Ring3State,
}

impl Ring3Domain {
    pub fn new(entry: UserEntry) -> Result<Self, Ring3Error> {
        if !entry.is_valid() {
            return Err(Ring3Error::InvalidEntry);
        }

        Ok(Self {
            id: Ring3DomainId::new(1),
            entry,
            state: Ring3State::Created,
        })
    }

    pub const fn id(&self) -> Ring3DomainId {
        self.id
    }

    pub const fn entry(&self) -> UserEntry {
        self.entry
    }

    pub const fn state(&self) -> Ring3State {
        self.state
    }

    pub fn make_ready(&mut self) {
        self.state = Ring3State::Ready;
    }

    pub fn mark_running(&mut self) {
        self.state = Ring3State::Running;
    }

    pub fn block(&mut self) {
        self.state = Ring3State::Blocked;
    }

    pub fn stop(&mut self) {
        self.state = Ring3State::Stopped;
    }

    pub fn crash(&mut self) {
        self.state = Ring3State::Crashed;
    }

    pub fn restart(&mut self) -> Result<(), Ring3Error> {
        if self.state != Ring3State::Crashed {
            return Err(Ring3Error::NotCrashed);
        }

        self.state = Ring3State::Ready;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ring3Error {
    InvalidEntry,
    NotCrashed,
}
