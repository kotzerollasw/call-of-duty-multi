use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use std::process::Command;
use std::sync::{Arc, Mutex};
use log::{info, error};

struct App {
    window: Window,
    status_label: Label,
}

impl App {
    fn new() -> Self {
        gtk::init().expect("Failed to initialize GTK.");
        let window = Window::new(WindowType::Toplevel);
        let status_label = Label::new(Some("Status: Ready"));
        window.set_title("Call of Duty Multi");
        window.set_default_size(400, 200);
        window.set_border_width(10);
        window.add(&status_label);
        window.show_all();
        App { window, status_label }
    }

    fn run(&self) {
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        gtk::main();
    }

    fn execute_command(&self, command: &str) {
        let output = Command::new(command).output();
        match output {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                info!("Command executed: {}", command);
                self.status_label.set_text(&result);
            }
            Err(e) => {
                error!("Failed to execute command: {}", e);
                self.status_label.set_text("Error executing command");
            }
        }
    }
}

fn main() {
    let app = App::new();
    app.run();
}