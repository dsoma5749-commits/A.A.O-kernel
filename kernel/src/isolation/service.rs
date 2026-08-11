#![allow(dead_code)]

use super::capability::{
    Capability, CapabilityId, CapabilityType, ServiceCapability, ServiceResource,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ServiceState {
    Created = 0,
    Running = 1,
    Stopped = 2,
    Crashed = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ServiceId(u64);

impl ServiceId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

pub struct UserService {
    id: ServiceId,
    state: ServiceState,
    capability: ServiceCapability,
}

impl UserService {
    pub const fn new(id: ServiceId, capability: ServiceCapability) -> Self {
        Self {
            id,
            state: ServiceState::Created,
            capability,
        }
    }

    pub fn start(&mut self) -> Result<(), ServiceError> {
        match self.state {
            ServiceState::Created | ServiceState::Stopped => {
                self.state = ServiceState::Running;
                Ok(())
            }

            ServiceState::Running => Err(ServiceError::AlreadyRunning),

            ServiceState::Crashed => Err(ServiceError::Crashed),
        }
    }

    pub fn stop(&mut self) {
        self.state = ServiceState::Stopped;
    }

    pub fn mark_crashed(&mut self) {
        self.state = ServiceState::Crashed;
    }

    pub fn restart(&mut self) -> Result<(), ServiceError> {
        if self.state != ServiceState::Crashed {
            return Err(ServiceError::NotCrashed);
        }

        self.state = ServiceState::Running;
        Ok(())
    }

    pub const fn id(&self) -> ServiceId {
        self.id
    }

    pub const fn state(&self) -> ServiceState {
        self.state
    }

    pub const fn capability(&self) -> &ServiceCapability {
        &self.capability
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceError {
    AlreadyRunning,
    Crashed,
    NotCrashed,
}

pub const fn service_capability(id: u64) -> ServiceCapability {
    let capability = Capability::new(
        CapabilityId::new(id),
        CapabilityType::Service,
        Capability::READ | Capability::WRITE,
    );

    super::capability::CapabilityToken::<ServiceResource>::new(capability)
}
