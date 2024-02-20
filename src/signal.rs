#[derive(Default, PartialEq, Eq)]
pub enum OnceSignal {
    #[default]
    Down,
    Up,
    Off,
}

impl OnceSignal {
    pub fn signal(&mut self) {
        if *self != OnceSignal::Off {
            *self = OnceSignal::Up
        }
    }

    pub fn try_take(&mut self) -> bool {
        if *self == OnceSignal::Up {
            *self = OnceSignal::Off;
            true
        } else {
            false
        }
    }

    pub fn is_off(&self) -> bool {
        *self == OnceSignal::Off
    }
}

