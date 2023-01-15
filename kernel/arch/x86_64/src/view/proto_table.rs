use super::prelude::*;
use alloc::vec::Vec;
use uefi::{prelude::BootServices, table::boot::SearchType, Handle};

#[derive(Default)]
pub struct ProtoTableView {
    protos: Option<Vec<Handle>>,
    index: usize,
    offset: usize,
}

impl ProtoTableView {
    pub(super) fn show(&mut self, table: &TableTy) {
        let protos = self
            .protos
            .get_or_insert_with(|| load_protos(table.boot_services()));

        println!("< Protocols");
        println!();
        let height = unsafe { &mut *uefi_services::system_table().as_mut() }
            .stdout()
            .current_mode()
            .unwrap()
            .unwrap()
            .rows();
        for (idx, handle) in protos.iter().enumerate().skip(self.offset).take(height - 5) {
            print!("{}", if idx == self.index { "> " } else { "  " });
            print!("{idx:02} {handle:?}");
            let protocols = table.boot_services().protocols_per_handle(*handle).unwrap();
            print!(" ({} protocols", protocols.protocols().len());
            for (idx, protocol) in protocols.protocols().iter().enumerate() {
                print!("{}", if idx == 0 { "; " } else { ", " });
                if let Some(name) = guid_to_name(**protocol) {
                    print!("{name}");
                } else {
                    print!("Unknown");
                }
            }
            println!(")");
        }
    }
    pub(super) fn update(&mut self, table: &TableTy, key: Key) -> UpdateResult {
        let len = self.protos.as_ref().map(|p| p.len()).unwrap_or(0);
        match key {
            Key::Special(ScanCode::UP) if self.index > 0 => {
                self.index -= 1;
                UpdateResult::Render
            }
            Key::Special(ScanCode::DOWN) if self.index + 1 < len => {
                self.index += 1;
                UpdateResult::Render
            }
            Key::Special(ScanCode::LEFT) => UpdateResult::NewView(View::ConfigTable(
                super::config_table::ConfigTableView::default(),
            )),
            Key::Special(ScanCode::PAGE_DOWN) if self.offset < len - 10 => {
                self.offset += 10;
                UpdateResult::Render
            }
            Key::Special(ScanCode::PAGE_UP) => {
                self.offset = if self.offset < 10 {
                    0
                } else {
                    self.offset - 10
                };
                UpdateResult::Render
            }
            #[cfg(feature = "qemu")]
            Key::Special(ScanCode::ESCAPE) => {
                use qemu_exit::{QEMUExit, X86};
                X86::new(0xF4, 1).exit_success();
            }
            _ => UpdateResult::None,
        }
    }
}

fn load_protos(boot: &BootServices) -> Vec<Handle> {
    // Search by protocol.
    let search_type = SearchType::AllHandles;

    // Determine how much we need to allocate.
    let buffer_size = boot.locate_handle(search_type, None).unwrap();

    // Allocate a large enough buffer without pointless initialization.
    let mut handles = Vec::with_capacity(buffer_size);
    let buffer = handles.spare_capacity_mut();

    // Perform the search.
    let buffer_size = boot.locate_handle(search_type, Some(buffer)).unwrap();

    // Mark the returned number of elements as initialized.
    unsafe {
        handles.set_len(buffer_size);
    }

    handles
}

macro_rules! impl_protocols {
    ($($full_path:ty => $name:ident), *) => {
        enum Protocols {
            $($name, )*
        }

        impl Protocols {
            pub fn all() -> &'static [Protocols] {
                &[
                    $(Self::$name, )*
                ]
            }

            pub fn guid(&self) -> uefi::Guid {
                match self {
                    $(
                        Self::$name => <$full_path as uefi::Identify>::GUID,
                    )*
                }
            }
        }
    };
}

impl_protocols! {
    uefi::proto::console::text::Input => Input,
    uefi::proto::debug::DebugSupport => DebugSupport,
    uefi::proto::device_path::DevicePath => DevicePath,
    uefi::proto::device_path::text::DevicePathFromText => DevicePathFromText,
    uefi::proto::device_path::text::DevicePathToText => DevicePathToText,
    uefi::proto::loaded_image::LoadedImage => LoadedImage,
    uefi::proto::media::block::BlockIO => BlockIO,
    uefi::proto::media::disk::DiskIo2 => DiskIo2,
    uefi::proto::media::disk::DiskIo => DiskIo,
    uefi::proto::media::file::FileInfo => FileInfo,
    uefi::proto::media::file::FileSystemInfo => FileSystemInfo,
    uefi::proto::media::file::FileSystemVolumeLabel => FileSystemVolumeLabel,
    uefi::proto::media::fs::SimpleFileSystem => SimpleFileSystem,
    uefi::proto::media::partition::PartitionInfo => PartitionInfo,
    uefi::proto::network::pxe::BaseCode => BaseCode,
    uefi::proto::pi::mp::MpServices => MpServices,
    uefi::proto::rng::Rng => Rng,
    uefi::proto::security::MemoryProtection => MemoryProtection,
    uefi::proto::shim::ShimLock => ShimLock,
    uefi::proto::string::unicode_collation::UnicodeCollation => UnicodeCollation,
    uefi::proto::console::gop::GraphicsOutput => GraphicsOutput,
    uefi::proto::console::pointer::Pointer => Pointer,
    uefi::proto::console::serial::Serial => Serial,
    uefi::proto::console::text::Output => Output
}
