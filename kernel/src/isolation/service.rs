use super::capability::Capability;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceId(pub u64);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceStatus {
    Stopped,
    Running,
    Crashed,
    Restarting,
}

#[allow(dead_code)]
pub struct UserService {
    pub id: ServiceId,
    pub status: ServiceStatus,
    pub cap: Capability,
    pub restart_count: u32,
}

#[allow(dead_code)]
impl UserService {
    pub fn new(id: ServiceId, cap: Capability) -> Self {
        Self {
            id,
            status: ServiceStatus::Stopped,
            cap,
            restart_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.status = ServiceStatus::Running;
        Ok(())
    }

    /// Self-Healing Handler: Auto-reboot crashed user-space services
    pub fn handle_fault(&mut self) -> Result<(), &'static str> {
        self.status = ServiceStatus::Crashed;
        self.restart_count += 1;

        // Instant microkernel restart trigger
        self.status = ServiceStatus::Restarting;
        self.start()
    }
}

#[allow(dead_code)]
pub fn service_capability(id: u64) -> Capability {
    Capability::new(
        super::capability::CapabilityId(id),
        super::capability::CapabilityType::IpcEndpoint,
        Capability::READ | Capability::WRITE | Capability::EXECUTE,
    )
}
