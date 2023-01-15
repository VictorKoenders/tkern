mod config_table;
mod proto_table;

mod prelude {
    pub(super) use super::{guid_to_name, TableTy, UpdateResult, View};
    pub use uefi::proto::console::text::{Key, ScanCode};
    pub use uefi_services::{print, println};
}

use core::ffi::c_void;
use uefi::proto::console::text::Key;

type TableTy = uefi::table::SystemTable<uefi::table::Boot>;

pub struct State {
    table: TableTy,
    view: View,
}

enum View {
    ConfigTable(config_table::ConfigTableView),
    ProtoTable(proto_table::ProtoTableView),
}

impl State {
    pub fn new(_image: uefi::Handle, table: uefi::table::SystemTable<uefi::table::Boot>) -> Self {
        Self {
            table,
            view: View::ConfigTable(config_table::ConfigTableView::default()),
        }
    }

    pub fn run(mut self) -> ! {
        let input_event = unsafe { self.table.stdin().wait_for_key_event().unsafe_clone() };
        loop {
            self.table.stdout().clear().unwrap();
            self.view.show(&self.table);

            loop {
                let key = loop {
                    if let Some(key) = self.table.stdin().read_key().unwrap() {
                        break key;
                    }
                    self.table
                        .boot_services()
                        .wait_for_event(&mut [unsafe { input_event.unsafe_clone() }])
                        .unwrap();
                };

                match self.view.update(&self.table, key) {
                    UpdateResult::None => {}
                    UpdateResult::Render => break,
                    UpdateResult::NewView(view) => {
                        self.view = view;
                        break;
                    }
                }
            }
        }
    }
}

pub(self) enum UpdateResult {
    None,
    Render,
    NewView(View),
}

impl View {
    pub(self) fn try_from_guid_and_ptr(guid: uefi::Guid, ptr: *const c_void) -> Option<Self> {
        None
    }
    fn show(&mut self, table: &TableTy) {
        match self {
            View::ConfigTable(view) => view.show(table),
            View::ProtoTable(view) => view.show(table),
        }
    }
    fn update(&mut self, table: &TableTy, key: Key) -> UpdateResult {
        match self {
            View::ConfigTable(view) => view.update(table, key),
            View::ProtoTable(view) => view.update(table, key),
        }
    }
}

pub fn guid_to_name(guid: uefi::Guid) -> Option<&'static str> {
    use uefi::Identify;
    Some(match guid {
        uefi::proto::console::text::Input::GUID => "text::Input",
        uefi::proto::debug::DebugSupport::GUID => "debug::DebugSupport",
        uefi::proto::device_path::DevicePath::GUID => "device_path::DevicePath",
        uefi::proto::device_path::text::DevicePathFromText::GUID => {
            "device_path::text::DevicePathFromText"
        }
        uefi::proto::device_path::text::DevicePathToText::GUID => {
            "device_path::text::DevicePathToText"
        }
        uefi::proto::loaded_image::LoadedImage::GUID => "loaded_image::LoadedImage",
        uefi::proto::media::block::BlockIO::GUID => "media::block::BlockIO",
        uefi::proto::media::disk::DiskIo2::GUID => "media::disk::DiskIo2",
        uefi::proto::media::disk::DiskIo::GUID => "media::disk::DiskIo",
        uefi::proto::media::file::FileInfo::GUID => "media::file::FileInfo",
        uefi::proto::media::file::FileSystemInfo::GUID => "media::file::FileSystemInfo",
        uefi::proto::media::file::FileSystemVolumeLabel::GUID => {
            "media::file::FileSystemVolumeLabel"
        }
        uefi::proto::media::fs::SimpleFileSystem::GUID => "media::fs::SimpleFileSystem",
        uefi::proto::media::partition::PartitionInfo::GUID => "media::partition::PartitionInfo",
        uefi::proto::network::pxe::BaseCode::GUID => "network::pxe::BaseCode",
        uefi::proto::pi::mp::MpServices::GUID => "pi::mp::MpServices",
        uefi::proto::rng::Rng::GUID => "rng::Rng",
        uefi::proto::security::MemoryProtection::GUID => "security::MemoryProtection",
        uefi::proto::shim::ShimLock::GUID => "shim::ShimLock",
        uefi::proto::string::unicode_collation::UnicodeCollation::GUID => {
            "string::unicode_collation::UnicodeCollation"
        }
        uefi::proto::console::gop::GraphicsOutput::GUID => "gop::GraphicsOutput",
        uefi::proto::console::pointer::Pointer::GUID => "console::pointer::Pointer",
        uefi::proto::console::serial::Serial::GUID => "console::serial::Serial",
        uefi::proto::console::text::Output::GUID => "console::text::Output",

        uefi::table::cfg::ACPI2_GUID => "table::cfg::ACPI2_GUID",
        uefi::table::cfg::ACPI_GUID => "table::cfg::ACPI_GUID",
        uefi::table::cfg::DEBUG_IMAGE_INFO_GUID => "table::cfg::DEBUG_IMAGE_INFO_GUID",
        uefi::table::cfg::DXE_SERVICES_GUID => "table::cfg::DXE_SERVICES_GUID",
        uefi::table::cfg::HAND_OFF_BLOCK_LIST_GUID => "table::cfg::HAND_OFF_BLOCK_LIST_GUID",
        uefi::table::cfg::LZMA_COMPRESS_GUID => "table::cfg::LZMA_COMPRESS_GUID",
        uefi::table::cfg::MEMORY_STATUS_CODE_RECORD_GUID => {
            "table::cfg::MEMORY_STATUS_CODE_RECORD_GUID"
        }
        uefi::table::cfg::MEMORY_TYPE_INFORMATION_GUID => {
            "table::cfg::MEMORY_TYPE_INFORMATION_GUID"
        }
        uefi::table::cfg::PROPERTIES_TABLE_GUID => "table::cfg::PROPERTIES_TABLE_GUID",
        uefi::table::cfg::SMBIOS3_GUID => "table::cfg::SMBIOS3_GUID",
        uefi::table::cfg::SMBIOS_GUID => "table::cfg::SMBIOS_GUID",
        uefi::table::cfg::TIANO_COMPRESS_GUID => "table::cfg::TIANO_COMPRESS_GUID",

        EFI_MEMORY_ATTRIBUTES_TABLE => "EfiMemoryAttributesTable",
        _ => return None,
    })
}

const EFI_MEMORY_ATTRIBUTES_TABLE: uefi::Guid = uefi::Guid::from_bytes([
    0x1d, 0x91, 0xfa, 0xdc, 0xeb, 0x26, 0x9f, 0x46, 0xa2, 0x20, 0x38, 0xb7, 0xdc, 0x46, 0x12, 0x20,
]);
