use super::capability::IpcCapability;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EndpointId(pub u64);

#[allow(dead_code)]
pub struct IpcMessage {
    pub registers: [u64; 4], // RAX, RDI, RSI, RDX equivalents
    pub count: usize,
}

#[allow(dead_code)]
impl IpcMessage {
    pub fn empty() -> Self {
        Self {
            registers: [0; 4],
            count: 0,
        }
    }

    pub fn push(&mut self, val: u64) -> Result<(), &'static str> {
        if self.count < 4 {
            self.registers[self.count] = val;
            self.count += 1;
            Ok(())
        } else {
            Err("IPC Message Buffer Full")
        }
    }
}

#[allow(dead_code)]
pub struct IpcEndpoint {
    pub id: EndpointId,
}

#[allow(dead_code)]
impl IpcEndpoint {
    pub fn new(id: EndpointId) -> Self {
        Self { id }
    }

    /// Synchronous Send: Transfers registers securely across processes
    pub fn send(&self, cap: &IpcCapability, msg: &IpcMessage) -> Result<(), &'static str> {
        if !cap.cap.has_permission(super::capability::Capability::WRITE) {
            return Err("Access Denied: Missing WRITE permission for IPC");
        }

        // Fast In-Register Data Transfer simulation
        let _payload_sum = msg.registers.iter().sum::<u64>();
        Ok(())
    }
}
