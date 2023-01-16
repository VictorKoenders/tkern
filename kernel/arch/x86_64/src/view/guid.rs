#[derive(Debug)]
pub enum GuidType {
    Protocol(GuidProtocol),
    Config(GuidConfig),
    Unknown(uefi::Guid),
}
impl GuidType {
    pub fn from_guid(guid: uefi::Guid) -> Self {
        if let Some(protocol) = GuidProtocol::from_guid(guid) {
            Self::Protocol(protocol)
        } else if let Some(config) = GuidConfig::from_guid(guid) {
            Self::Config(config)
        } else {
            Self::Unknown(guid)
        }
    }
    /*pub const fn guid(&self) -> uefi::Guid {
        match self {
            Self::Unknown(g) => *g,
            Self::Config(p) => p.guid(),
            Self::Protocol(p) => p.guid(),
        }
    }*/
}

macro_rules! impl_guid_identity_variants {
    (enum $container:ident { $($ty:ty = $name:ident),* $(,)? }) => {
        #[derive(Debug)]
        pub enum $container {
            $( $name,)*
        }

        impl $container {
            pub fn from_guid(guid: uefi::Guid) -> Option<Self> {
                match guid {
                    $(<$ty as uefi::Identify>::GUID => Some(Self::$name),)*
                    _ => None
                }
            }
            pub fn all() -> &'static [Self] {
                &[
                    $(Self::$name,)*
                ]
            }
            pub const fn guid(&self) -> uefi::Guid {
                match self {
                    $(Self::$name => <$ty as uefi::Identify>::GUID,)*
                }
            }
        }
    }
}

impl_guid_identity_variants!(
    enum GuidProtocol {
        uefi::proto::console::text::Input = Input,
        uefi::proto::debug::DebugSupport = DebugSupport,
        uefi::proto::device_path::DevicePath = DevicePath,
        uefi::proto::device_path::text::DevicePathFromText = DevicePathFromText,
        uefi::proto::device_path::text::DevicePathToText = DevicePathToText,
        uefi::proto::loaded_image::LoadedImage = LoadedImage,
        uefi::proto::media::block::BlockIO = BlockIO,
        uefi::proto::media::disk::DiskIo2 = DiskIo2,
        uefi::proto::media::disk::DiskIo = DiskIo,
        uefi::proto::media::file::FileInfo = FileInfo,
        uefi::proto::media::file::FileSystemInfo = FileSystemInfo,
        uefi::proto::media::file::FileSystemVolumeLabel = FileSystemVolumeLabel,
        uefi::proto::media::fs::SimpleFileSystem = SimpleFileSystem,
        uefi::proto::media::partition::PartitionInfo = PartitionInfo,
        uefi::proto::network::pxe::BaseCode = BaseCode,
        uefi::proto::pi::mp::MpServices = MpServices,
        uefi::proto::rng::Rng = Rng,
        uefi::proto::security::MemoryProtection = MemoryProtection,
        uefi::proto::shim::ShimLock = ShimLock,
        uefi::proto::string::unicode_collation::UnicodeCollation = UnicodeCollation,
        uefi::proto::console::gop::GraphicsOutput = GraphicsOutput,
        uefi::proto::console::pointer::Pointer = Pointer,
        uefi::proto::console::serial::Serial = Serial,
        uefi::proto::console::text::Output = Output,
    }
);

macro_rules! impl_guid_variants {
    (enum $container:ident { $($name:ident = $val:expr),* $(,)? }) => {
        #[derive(Debug)]
        pub enum $container {
            $( $name,)*
        }

        impl $container {
            fn from_guid(guid: uefi::Guid) -> Option<Self> {
                match guid {
                    $(v if v == $val => Some(Self::$name),)*
                    _ => None
                }
            }
            /*const fn guid(&self) -> uefi::Guid {
                match self {
                    $(Self::$name => $val,)*
                }
            }*/
        }
    }
}

impl_guid_variants!(
    enum GuidConfig {
        ACPI2 = uefi::table::cfg::ACPI2_GUID,
        ACPI = uefi::table::cfg::ACPI_GUID,
        DebugImageInfo = uefi::table::cfg::DEBUG_IMAGE_INFO_GUID,
        DXEServices = uefi::table::cfg::DXE_SERVICES_GUID,
        HandOffBlockList = uefi::table::cfg::HAND_OFF_BLOCK_LIST_GUID,
        LZMACompress = uefi::table::cfg::LZMA_COMPRESS_GUID,
        MemoryStatusCodeRecord = uefi::table::cfg::MEMORY_STATUS_CODE_RECORD_GUID,
        MemoryTypeInformation = uefi::table::cfg::MEMORY_TYPE_INFORMATION_GUID,
        PropertiesTable = uefi::table::cfg::PROPERTIES_TABLE_GUID,
        SMBIOS3 = uefi::table::cfg::SMBIOS3_GUID,
        SMBIOS = uefi::table::cfg::SMBIOS_GUID,
        TianoCompress = uefi::table::cfg::TIANO_COMPRESS_GUID,
    }
);

const EFI_MEMORY_ATTRIBUTES_TABLE: uefi::Guid = uefi::Guid::from_bytes([
    0x1d, 0x91, 0xfa, 0xdc, 0xeb, 0x26, 0x9f, 0x46, 0xa2, 0x20, 0x38, 0xb7, 0xdc, 0x46, 0x12, 0x20,
]);
