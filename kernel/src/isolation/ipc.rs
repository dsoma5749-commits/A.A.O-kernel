#![allow(dead_code)]

use super::capability::{CapabilityToken, IpcResource};

pub const IPC_MAX_WORDS: usize = 8;

#[derive(Clone, Copy)]
pub struct IpcMessage {
    words: [u64; IPC_MAX_WORDS],
    len: usize,
}

impl IpcMessage {
    pub const fn empty() -> Self {
        Self {
            words: [0; IPC_MAX_WORDS],
            len: 0,
        }
    }

    pub fn push(&mut self, value: u64) -> bool {
        if self.len >= IPC_MAX_WORDS {
            return false;
        }

        self.words[self.len] = value;
        self.len += 1;
        true
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn get(&self, index: usize) -> Option<u64> {
        if index >= self.len {
            None
        } else {
            Some(self.words[index])
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct EndpointId(u64);

impl EndpointId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

pub struct IpcEndpoint {
    id: EndpointId,
}

impl IpcEndpoint {
    pub const fn new(id: EndpointId) -> Self {
        Self { id }
    }

    pub const fn id(&self) -> EndpointId {
        self.id
    }

    pub fn send(
        &self,
        capability: &CapabilityToken<IpcResource>,
        message: &IpcMessage,
    ) -> Result<(), IpcError> {
        if capability.capability().id().raw() != self.id.raw() {
            return Err(IpcError::InvalidCapability);
        }

        if message.len() > IPC_MAX_WORDS {
            return Err(IpcError::MessageTooLarge);
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpcError {
    InvalidCapability,
    MessageTooLarge,
    EndpointUnavailable,
}
