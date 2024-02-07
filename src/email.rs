use std::io::{self, Read, Stdin, Write};

use lettre::{message::{Mailbox, MessageBuilder}, transport::smtp::authentication::{Credentials, Mechanism}, Message, SmtpTransport, Transport};
use colored::Colorize;

use crate::relays::Relay;

pub struct Sender {
    email: Mailbox,
    password: String
}

impl Sender {
    pub fn new<S: ToString>(email: S, password: S) -> anyhow::Result<Self> {
        Ok(Self {
            email: email.to_string().parse()?,
            password: password.to_string()
        })
    }
    pub fn from_stdin() -> anyhow::Result<Self> {
        Ok(Self {
            email: inquire::Text::new("Enter your email").prompt()?.parse()?,
            password: rpassword::prompt_password("Enter your password(will be hidden):")?
        })
    }

    pub fn email(&self) -> &Mailbox {
        &self.email
    }

    /// Create message builder
    pub fn message(&self) -> anyhow::Result<MessageBuilder> {
        Ok(Message::builder()
                    .from(self.email.clone()))
    }
    /// Construct new credentials
    pub fn credentials(&self) -> Credentials {
        Credentials::new(self.email.to_string(), self.password.clone())
    }

    pub fn mailer(&self, credentials: Credentials, relay: Relay) -> anyhow::Result<SmtpTransport> {
        let (port, addr, mechanisms) = relay.info().destructure();

        println!("\ncontacting relay \"{}\" on port {}", addr.green(), port.to_string().blue());
        
        let mailer = SmtpTransport::starttls_relay(addr)?
            .port(port)
            .credentials(credentials) 
            .authentication(Vec::from(mechanisms))
            .build();

        Ok(mailer)
    }
}