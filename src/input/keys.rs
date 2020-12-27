use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = KeyEnum)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
/// Rust representation of a set of common keys.
/// The names match the [Key Code Values](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values).
/// The keys listed should all have the same representation on all platforms.
///
/// For older browsers that don't support the `code` value, a conversion from the `key` value is done with best effort. This may not consider keyboard layouts perfectly.
pub enum Key {
    // These values are the same in event.key and event.code
    Escape,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,
    Enter,
    Tab,
    Backspace,
    Delete,
    // These values need to be mapped from event.key to event.code for compatibility
    Space,
    AltLeft,
    AltRight,
    ShiftLeft,
    ShiftRight,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
}