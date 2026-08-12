#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapabilityId(pub u64);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilityType {
    MemoryRegion,
    IpcEndpoint,
    HardwareInterrupt,
}

#[allow(dead_code)]
pub struct Capability {
    pub id: CapabilityId,
    pub cap_type: CapabilityType,
    pub permissions: u32,
}

#[allow(dead_code)]
impl Capability {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXECUTE: u32 = 1 << 2;
    pub const GRANT: u32 = 1 << 3;

    pub fn new(id: CapabilityId, cap_type: CapabilityType, permissions: u32) -> Self {
        Self {
            id,
            cap_type,
            permissions,
        }
    }

    pub fn has_permission(&self, perm: u32) -> bool {
        (self.permissions & perm) == perm
    }
}

#[allow(dead_code)]
pub struct IpcCapability {
    pub cap: Capability,
}

#[allow(dead_code)]
impl IpcCapability {
    pub fn new(cap: Capability) -> Self {
        Self { cap }
    }
}
