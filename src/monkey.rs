use std::{thread, time};
use inputbot::{KeybdKey, MouseButton, MouseCursor};

pub struct MonkeyInstance {
    x: i32,
    y: i32,
}

impl MonkeyInstance {
    pub fn new() -> Self {
        MonkeyInstance { x: 0, y: 0 }
    }

    pub fn replace(&mut self, monkey: MonkeyInstance) {
        self.x = monkey.x;
        self.y = monkey.y;
    }
}

pub enum Monkeys {
    Hero,
    Ace,
    TackShooter,
    Heli
}

pub enum UpgradePath {
    Top,
    Middle,
    Bottom,
}

pub fn spawn_monkey(x: i32, y: i32, m: Monkeys) -> MonkeyInstance {
    let key: KeybdKey = match m {
        Monkeys::Hero => KeybdKey::UKey,
        Monkeys::Ace => KeybdKey::VKey,
        Monkeys::TackShooter => KeybdKey::RKey,
        Monkeys::Heli => KeybdKey::BKey
    };

    key.press();
    key.release();
    thread::sleep(time::Duration::from_millis(100));

    left_click(x, y);

    MonkeyInstance { x, y }
}

pub fn upgrade_monkey(m: &MonkeyInstance, path: Vec<UpgradePath>) {
    left_click(m.x, m.y);

    for p in path.iter() {
        let key: KeybdKey = match p {
            UpgradePath::Top => KeybdKey::CommaKey,
            UpgradePath::Middle => KeybdKey::PeriodKey,
            UpgradePath::Bottom => KeybdKey::SlashKey,
        };

        key.press();
        key.release();
        thread::sleep(time::Duration::from_millis(100));
    }

    left_click(60, 500);
}

pub fn left_click(x: i32, y: i32) {
    MouseCursor::move_abs(x, y);
    thread::sleep(time::Duration::from_millis(50));

    MouseButton::LeftButton.press();
    MouseButton::LeftButton.release();
    thread::sleep(time::Duration::from_millis(50));
}