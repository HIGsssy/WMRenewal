/// UI event types translated from SDL2 events.
#[derive(Debug, Clone)]
pub enum UiEvent {
    MouseClick { x: i32, y: i32 },
    MouseDown { x: i32, y: i32 },
    MouseUp { x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    MouseWheelUp,
    MouseWheelDown,
    KeyPress { key: char, shift: bool },
    KeyDown { keycode: sdl2::keyboard::Keycode },
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Enter,
    Escape,
    Quit,
}
