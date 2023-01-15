use uefi::{
    proto::console::text::{Key, ScanCode},
    Char16,
};
use uefi_services::print;

pub struct ListOptions<'a, T, F, SEL> {
    pub entries: &'a [T],
    pub render: F,
    pub select: SEL,
}

pub fn list<'table, 'entries, T, F, SEL>(
    table: &'table mut uefi::table::SystemTable<uefi::table::Boot>,
    mut opts: ListOptions<'entries, T, F, SEL>,
) where
    T: 'static,
    F: FnMut(&mut uefi::table::SystemTable<uefi::table::Boot>, &T),
    SEL: FnMut(&mut uefi::table::SystemTable<uefi::table::Boot>, &T),
{
    let input_event = unsafe { table.stdin().wait_for_key_event().unsafe_clone() };
    let mut index = 0;
    let mut offset = 0;

    let return_char = Char16::try_from('\r').unwrap();
    let index_width = if opts.entries.len() + 1 < 10 {
        1
    } else if opts.entries.len() + 1 < 100 {
        2
    } else {
        3
    };

    loop {
        table.stdout().clear().unwrap();
        for (idx, item) in opts.entries.iter().enumerate() {
            print!(
                "{} {:0>width$} ",
                if idx == index { '>' } else { ' ' },
                idx + 1,
                width = index_width
            );
            (opts.render)(table, item);
        }

        loop {
            let key = loop {
                if let Some(key) = table.stdin().read_key().unwrap() {
                    break key;
                }
                table
                    .boot_services()
                    .wait_for_event(&mut [unsafe { input_event.unsafe_clone() }])
                    .unwrap();
            };

            match key {
                Key::Printable(c) if c == return_char => {
                    (opts.select)(table, &opts.entries[index]);
                    break;
                }
                Key::Special(ScanCode::UP) if index > 0 => {
                    index -= 1;
                    break;
                }
                Key::Special(ScanCode::DOWN) if index + 1 < opts.entries.len() => {
                    index += 1;
                    break;
                }
                Key::Special(ScanCode::ESCAPE) => return,
                _ => {}
            }
        }
    }
}
