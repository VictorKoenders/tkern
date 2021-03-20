use crate::futures::io::KeyboardModifiers;
use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

struct KeyboardState {
    state: Keyboard<layouts::Us104Key, ScancodeSet1>,
    modifiers: KeyboardModifiers,
}
impl KeyboardState {
    pub fn new() -> Self {
        Self {
            state: Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore),
            modifiers: KeyboardModifiers::default(),
        }
    }

    pub fn update(&mut self, scancode: u8) -> Option<DecodedKey> {
        if let Ok(Some(key_event)) = self.state.add_byte(scancode) {
            if key_event.code == KeyCode::ControlLeft || key_event.code == KeyCode::ControlRight {
                self.modifiers.ctrl_pressed = key_event.state == KeyState::Down;
            }
            if let Some(key) = self.state.process_keyevent(key_event) {
                return Some(key);
            }
        }
        None
    }
}

lazy_static! {
    static ref KEYBOARD: Mutex<KeyboardState> = Mutex::new(KeyboardState::new());
}
static ENTER_PRESSED_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Puts the CPU to sleep until the enter key is pressed.
pub fn wait_for_enter() {
    let count = ENTER_PRESSED_COUNT.load(Ordering::Relaxed);
    while count == ENTER_PRESSED_COUNT.load(Ordering::Relaxed) {
        crate::arch::halt();
    }
}

/// Called when a keyboard button is pressed.
pub fn keyboard_interrupt(_stack_frame: super::StackFrame, scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Some(key) = keyboard.update(scancode) {
        match key {
            DecodedKey::Unicode(character) => {
                crate::futures::RUNTIME.notify_key_pressed(character, keyboard.modifiers);
            }
            DecodedKey::RawKey(key) => vga_print!("{:?}", key),
        }
    }
}
