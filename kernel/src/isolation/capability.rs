#![allow(dead_code)]

use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CapabilityType {
    Memory = 1,
    IpcEndpoint = 2,
    Service = 3,
    Device = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CapabilityId(u64);

impl CapabilityId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capability {
    id: CapabilityId,
    kind: CapabilityType,
    rights: u32,
}

impl Capability {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXECUTE: u32 = 1 << 2;
    pub const GRANT: u32 = 1 << 3;

    pub const fn new(id: CapabilityId, kind: CapabilityType, rights: u32) -> Self {
        Self { id, kind, rights }
    }

    pub const fn id(self) -> CapabilityId {
        self.id
    }

    pub const fn kind(self) -> CapabilityType {
        self.kind
    }

    pub const fn rights(self) -> u32 {
        self.rights
    }

    pub const fn allows(self, required: u32) -> bool {
        (self.rights & required) == required
    }
}

pub struct CapabilityToken<T> {
    capability: Capability,
    _marker: PhantomData<T>,
}

impl<T> CapabilityToken<T> {
    pub const fn new(capability: Capability) -> Self {
        Self {
            capability,
            _marker: PhantomData,
        }
    }

    pub const fn capability(&self) -> Capability {
        self.capability
    }

    pub const fn allows(&self, required: u32) -> bool {
        self.capability.allows(required)
    }
}

pub enum MemoryResource {}
pub enum IpcResource {}
pub enum ServiceResource {}
pub enum DeviceResource {}

pub type MemoryCapability = CapabilityToken<MemoryResource>;
pub type IpcCapability = CapabilityToken<IpcResource>;
pub type ServiceCapability = CapabilityToken<ServiceResource>;
pub type DeviceCapability = CapabilityToken<DeviceResource>;
