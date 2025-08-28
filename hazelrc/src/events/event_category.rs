use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EventCategory: u8 {
        const NONE           = 0;
        const INPUT          = 1 << 0;
        const KEYBOARD       = 1 << 1;
        const MOUSE          = 1 << 2;
        const MOUSE_BUTTON   = 1 << 3;
    }
}