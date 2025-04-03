use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Features {
    aimbot: bool,
    esp: bool,
    wallhack: bool,
    speedhack: f32,
}

impl Features {
    pub fn new() -> Self {
        Features {
            aimbot: false,
            esp: false,
            wallhack: false,
            speedhack: 1.0,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot = !self.aimbot;
    }

    pub fn toggle_esp(&mut self) {
        self.esp = !self.esp;
    }

    pub fn toggle_wallhack(&mut self) {
        self.wallhack = !self.wallhack;
    }

    pub fn set_speedhack(&mut self, speed: f32) {
        self.speedhack = speed;
    }
}