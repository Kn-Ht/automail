use std::io::{self, Read, Write};

use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport};
use lettre::Transport;
mod relays;

mod email;
use email::Sender;
use colored::Colorize;

mod config;

use crate::relays::Relay;

const DEBUG: bool = cfg!(debug_assertions);

fn pause_and_quit(msg: &str, code: i32) -> ! {
    print!("{}", msg.bold());
    let _ = io::stdout().flush();
    let _ = io::stdin().read(&mut [0u8]);
    std::process::exit(code);
}

fn main_() -> anyhow::Result<()> {
    // 'Sender' stores information relating to the user.
    let sender = if DEBUG {
        Sender::new("testgosro1@cigma.nl", "oO92m8-OWn$7lbW8gQn!y")?
    } else {
        Sender::from_stdin()?
    };
    
    // Read receiver data
    let receiver = inquire::Text::new("Enter the receiver's email").prompt()?;

    println!("----------------------------");
    println!("Sender: {}", sender.email().to_string().green());

    let email = sender.message()?
        .to(receiver.parse()?)
        .subject("The Subject")
        .body("Hello, World!".to_owned())?;

    let credentials = sender.credentials();
    let mailer = sender.mailer(credentials, Relay::Outlook)?;

    match mailer.send(&email) {
        Ok(_) => {
            println!("{} {}", "Successfully sent email to ".bold(), receiver.green());
        }
        Err(e) => {
            eprintln!("Failed to send email: {}", e.to_string().red());
        }
    }

    Ok(())
}

fn main() {
    let mut exit_code = 0;
    if let Err(e) = main_() {
        eprintln!("{}: {e}", "error".bright_red());
        exit_code = 1;
    }
    pause_and_quit("Press any key to exit . . .", exit_code);
}