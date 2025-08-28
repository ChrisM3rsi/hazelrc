#[derive(Copy, Clone, Debug)]
pub enum EventType {
    None = 0,
    WindowClose,
    WindowResize,
    AppTick,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}