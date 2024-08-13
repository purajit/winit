#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum DeviceId {
    Wheel,
    Keyboard,
    Id(i32),
}

impl DeviceId {
    pub fn new(pointer_id: i32) -> Self {
        Self::Id(pointer_id)
    }

    #[cfg(test)]
    pub const fn dummy() -> Self {
        Self::Id(-1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FingerId {
    pointer_id: i32,
    primary: bool,
}

impl FingerId {
    pub fn new(pointer_id: i32, primary: bool) -> Self {
        Self { pointer_id, primary }
    }

    #[cfg(test)]
    pub const fn dummy() -> Self {
        Self { pointer_id: -1, primary: false }
    }

    pub fn is_primary(self) -> bool {
        self.primary
    }
}
