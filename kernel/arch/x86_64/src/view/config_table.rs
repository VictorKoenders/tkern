use super::prelude::*;

#[derive(Default)]
pub struct ConfigTableView {
    index: usize,
    error: Option<&'static str>,
}

impl ConfigTableView {
    pub(super) fn show(&mut self, table: &TableTy) {
        println!("  Config table >");
        println!();
        for (idx, entry) in table.config_table().iter().enumerate() {
            print!("{}", if self.index == idx { "> " } else { "  " });
            print!("guid: {}, address: {:p}", entry.guid, entry.address);
            if let Some(entry) = guid_to_name(entry.guid) {
                println!(", Type: {entry}");
            } else {
                println!();
            }
        }

        if let Some(error) = self.error.take() {
            println!();
            println!("{error}");
        }
    }

    pub(super) fn update(&mut self, table: &TableTy, key: Key) -> UpdateResult {
        match key {
            Key::Printable(key) if key.is('\r') => {
                // enter
                if let Some(config) = table.config_table().get(self.index) {
                    if let Some(view) = View::try_from_guid_and_ptr(config.guid, config.address) {
                        return UpdateResult::NewView(view);
                    } else {
                        self.error = Some("Unknown type; could not construct view");
                    }
                } else {
                    self.error = Some("Config table entry not found");
                }
                UpdateResult::Render
            }
            Key::Special(ScanCode::UP) if self.index > 0 => {
                self.index -= 1;
                UpdateResult::Render
            }
            Key::Special(ScanCode::DOWN) if self.index + 1 < table.config_table().len() => {
                self.index += 1;
                UpdateResult::Render
            }
            Key::Special(ScanCode::RIGHT) => UpdateResult::NewView(View::ProtoTable(
                super::proto_table::ProtoTableView::default(),
            )),
            #[cfg(feature = "qemu")]
            Key::Special(ScanCode::ESCAPE) => {
                use qemu_exit::{QEMUExit, X86};
                X86::new(0xF4, 1).exit_success();
            }
            _ => UpdateResult::None,
        }
    }
}

trait CharIs {
    fn is(&self, char: char) -> bool;
}

impl CharIs for uefi::Char16 {
    fn is(&self, char: char) -> bool {
        if let Ok(char) = uefi::Char16::try_from(char) {
            char == *self
        } else {
            false
        }
    }
}
