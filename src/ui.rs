use std::{cell::RefCell, rc::Rc};

use cursive::{
    event::EventResult,
    theme::Color,
    utils::markup::StyledString,
    view::{Nameable, Resizable, SizeConstraint},
    views::{Checkbox, DebugView, Dialog, EditView, LinearLayout, Panel, TextView},
    Cursive, View,
};

use crate::config::{Config, ConfigHandle, Server};

pub struct Ui {
    siv: cursive::CursiveRunnable,
}

impl Ui {
    pub fn init(config: ConfigHandle) -> anyhow::Result<Self> {
        Ok(Self {
            siv: {
                let mut siv = cursive::default();
                let _ = siv.load_toml(include_str!("cursive_style.toml"));
                siv.set_user_data(config);
                siv
            },
        })
    }
    /// Ask the user whether they REALLY want to exit
    fn ask_user_exit(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .dismiss_button("No")
                .button("Yes", |s| s.quit())
                .content(TextView::new("Do you really want to quit?"))
                .title("Quit?"),
        )
    }
    fn error(s: &mut Cursive, title: &str, content: &str) {
        s.add_layer(Dialog::text(content).dismiss_button("Okay").title(title));
    }
    /// Prompt user to enter login details before using the app
    pub fn show_login_dialog(&mut self) {
        let header_color = Color::Rgb(85, 85, 255);

        self.siv.add_layer(
            Dialog::new()
                .title("Login Details")
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new(StyledString::styled(
                            "Login details",
                            header_color,
                        )))
                        .child(
                            LinearLayout::horizontal()
                                .child(TextView::new("Username:  "))
                                .child(EditView::new().with_name("username").fixed_width(33)),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(TextView::new("Password:  "))
                                .child(EditView::new().fixed_width(33).with_name("password")),
                        )
                        .child(TextView::new(StyledString::styled("Server", header_color)))
                        .child(
                            LinearLayout::horizontal()
                                .child(TextView::new("TLS Enabled:  "))
                                .child(Checkbox::new().checked().with_name("tls")),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(TextView::new("SMTP Server: "))
                                .child(
                                    EditView::new()
                                        .content("smtp.office365.org")
                                        .fixed_width(33)
                                        .with_name("smtp"),
                                ),
                        ),
                )
                .button("Save", |s| {
                    let username = s
                        .call_on_name("username", |view: &mut EditView| view.get_content().clone())
                        .map(|content_rc| Rc::try_unwrap(content_rc).unwrap_or_default())
                        .unwrap_or_else(|| {
                            msgbox::create("A", "A", msgbox::IconType::Error);
                            "".into()
                        });

                    let password = s
                        .call_on_name("password", |view: &mut EditView| view.get_content())
                        .map(|content_rc| Rc::try_unwrap(content_rc).unwrap_or_default())
                        .unwrap_or_default();

                    let tls = s
                        .call_on_name("tls", |view: &mut Checkbox| view.is_checked())
                        .unwrap_or_default();

                    let smtp = s
                        .call_on_name("smtp", |view: &mut EditView| view.get_content())
                        .map(|content_rc| Rc::try_unwrap(content_rc).unwrap_or_default())
                        .unwrap_or_default();

                    if username.is_empty() {
                        Self::error(s, "Login Error", "Username cannot be empty.");
                    } else if password.is_empty() {
                        Self::error(s, "Login Error", "Password cannot be empty.");
                    } else if smtp.is_empty() {
                        Self::error(s, "Login Error", "SMTP address cannot be empty");
                    } else {
                        if let Some(ref mut conf) = s.user_data::<ConfigHandle>() {
                            //conf.conf.server = Some(Server::new(tls, ))
                        }

                        s.pop_layer();
                    }
                })
                .button("Exit", Self::ask_user_exit)
                .min_width(50),
        )
    }

    /// Create the elements, shows nothing until `.run()` is called
    pub fn load_interface(&mut self) {
        // Initialize the logger
        cursive::logger::init();

        let mut sel_view = cursive::views::SelectView::new();
        sel_view.add_all_str(vec!["Hello", "World!"]);

        self.siv.add_layer(
            LinearLayout::horizontal()
                .child(
                    Panel::new(
                        sel_view
                            .on_select(|s, item| {
                                let _ = msgbox::create("Hello", "Hello", msgbox::IconType::Info);
                            })
                            .full_width(),
                    )
                    .title("Clients"),
                )
                .child(
                    LinearLayout::vertical()
                        .child(Panel::new(cursive::views::TextArea::new()))
                        .child(Panel::new(DebugView::default())),
                )
                .full_screen(),
        );

        self.siv.add_global_callback('q', Self::ask_user_exit);

        if self
            .siv
            .user_data::<ConfigHandle>()
            .unwrap()
            .conf
            .server
            .is_none()
        {
            self.show_login_dialog();
        }
    }
    /// Start the ui loop
    pub fn run(&mut self) {
        self.siv.run();
    }
}
