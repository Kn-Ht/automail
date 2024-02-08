
use lettre::Transport;
use platform_dirs::AppDirs;
use std::{
    fs,
    io::{self, Read, Write},
};

mod relays;

mod email;
use colored::Colorize;
use email::Sender;

mod config;
use config::{Config, ConfigHandle};

use crate::{config::EXE_NAME, relays::Relay};

mod ui;
use ui::Ui;

// Name of the project
const DEBUG: bool = cfg!(debug_assertions);

fn main_() -> anyhow::Result<()> {
    // 'Sender' stores information relating to the user.
    let sender = if DEBUG {
        Sender::new("testgosro1@cigma.nl", "oO92m8-OWn$7lbW8gQn!y")?
    } else {
        Sender::from_stdin()?
    };

    // Config file
    let mut config = ConfigHandle::open()?;

    // UI
    let mut ui = Ui::init(config)?;
    ui.load_interface();
    ui.run();

    /*
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
    */

    Ok(())
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("{}: {e}", "error".bright_red());
        let _ = msgbox::create(EXE_NAME, &format!("Error: {e}"), msgbox::IconType::Error);
    }
}
