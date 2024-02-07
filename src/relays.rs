use lettre::transport::smtp::authentication::Mechanism;

pub struct RelayInfo {
    pub port: u16,
    pub addr: &'static str,
    pub mechanisms: &'static [Mechanism]
}

impl RelayInfo {
    const fn new(port: u16, addr: &'static str, mechanisms: &'static [Mechanism]) -> Self {
        Self {
            port, addr, mechanisms
        }
    }
    pub const fn destructure(self) -> (u16, &'static str, &'static [Mechanism]) {
        (self.port, self.addr, self.mechanisms)
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
pub enum Relay {
    Outlook
}

impl Relay {
    pub const fn info(&self) -> RelayInfo {
        match *self {
            Self::Outlook => RelayInfo::new(587, "smtp.office365.com", &[Mechanism::Login])
        }
    }
}