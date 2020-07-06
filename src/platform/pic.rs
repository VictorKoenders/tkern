use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use pic8259_simple::ChainedPics;
use spin::Mutex;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            layouts::Us104Key,
            ScancodeSet1,
            HandleControl::MapLettersToUnicode
        ));
}

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init() {
    unsafe { PICS.lock().initialize() }
}

pub fn interrupt_timer() {
    unsafe {
        notify_end_of_interrupt(InterruptIndex::Timer);
    }
}

pub fn interrupt_keyboard(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character == '\x1b' {
                        // escape
                        super::exit_qemu_success();
                    }
                    vga_println!("{}", character);
                }
                DecodedKey::RawKey(key) => {
                    if key == KeyCode::Escape {
                        super::exit_qemu_success();
                    }
                    vga_println!("{:?}", key);
                }
            }
        }
    }
    unsafe {
        notify_end_of_interrupt(InterruptIndex::Keyboard);
    }
}

unsafe fn notify_end_of_interrupt(index: InterruptIndex) {
    PICS.lock().notify_end_of_interrupt(index.as_u8())
}
