#[derive(Debug, Default, PartialEq, Eq)]
pub enum OnceSignalA {
    #[default]
    Down,
    Up,
    Off,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct OnceSignal {
    e: OnceSignalA,
}

impl OnceSignal {
    pub fn signal(&mut self) {
        self.e.signal()
    }

    pub fn try_take(&mut self) -> bool {
        self.e.try_take()
    }

    pub fn is_off(&self) -> bool {
        self.e.is_off()
    }
}

impl OnceSignalA {
    pub fn signal(&mut self) {
        if *self != Self::Off {
            *self = Self::Up
        }
    }

    pub fn try_take(&mut self) -> bool {
        if *self == Self::Up {
            *self = Self::Off;
            true
        } else {
            false
        }
    }

    pub fn is_off(&self) -> bool {
        *self == Self::Off
    }
}
