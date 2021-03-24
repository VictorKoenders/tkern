#![allow(dead_code, missing_docs, non_snake_case)]

#[repr(packed)]
pub struct Identify {
    pub config: IdentifyGeneralConfiguration, // u16
    pub num_cylinders: u16,
    pub special_configuration: u16,
    pub num_heads: u16,
    _retired_1: [u16; 2],
    pub num_secoors_per_track: u16,
    pub vendors_unique_1: [u16; 3],
    pub serial_number: [u8; 20],
    _retired_2: [u16; 2],
    _obsolete_1: u16,
    pub firmware_revision: [u8; 8],
    pub model_number: [u16; 20], // [u8; 40] but this doesn't implement `Default`
    pub maximum_block_transfer: u8,
    pub vendor_unique_2: u8,
    pub trusted_computing: TrustedComputing, // u16
    pub capabilities: Capabilities,          // [u16; 2]
    _obsolete_words_51: [u16; 2],
    pub translation_fields_valid: TranslationFieldsValid, // u8
    pub free_fall_control_sensitivity: u8,
    pub number_of_concurrent_cylinders: u16,
    pub number_of_concurrent_heads: u16,
    pub current_sectors_per_track: u16,
    pub current_sector_capacity: u32,
    pub current_multi_sector_setting: u8,
    pub nested_settings: NestedSettings, // u8
    pub user_addressable_sectors: u32,
    _obsolete_word_62: u16,
    pub multi_word_dma_support: u8,
    pub multi_word_dma_active: u8,
    pub advanced_pio_modes: u8,
    _reserved_byte_64: u8,
    pub maximum_mwxfer_cycle_time: u16,
    pub recommended_mwxfer_cycle_time: u16,
    pub minimum_pio_cycle_time: u16,
    pub minimum_pio_cycle_time_iordy: u16,
    pub additional_supported: AdditionalSupported, // u16
    _reserved_words_70: [u16; 5],
    pub queue_depth: QueueDepth,                          // u16
    pub serial_ata_capabilities: SerialAtaCapabilities,   // u32
    pub serial_ata_features_supported: SerialAtaFeatures, // u16
    pub serial_ata_features_enabled: SerialAtaFeatures,   // u16
    pub major_revision: u16,
    pub minor_revision: u16,
    pub command_set_supported: CommandSet, // [u8; 6]
    pub command_set_active: CommandSet,    // [u8; 6]
    pub ultra_dma_support: u8,
    pub ultra_dma_active: u8,
    pub normal_security_erase_unit: EraseUnit,   // u16
    pub enhanced_security_erase_unit: EraseUnit, // u16
    pub current_apm_level: u8,
    _reserved_word_91: u8,
    pub master_password_id: u16,
    pub hardware_reset_result: u16,
    pub current_acoustic_value: u8,
    pub recommended_acoustic_value: u8,
    pub stream_min_request_size: u16,
    pub streaming_transfer_time_dma: u16,
    pub streaming_access_latency_dma_pio: u16,
    pub streaming_perf_granularity: u32,
    pub max_48_bit_lba: [u32; 2],
    pub streaming_transfer_time: u16,
    pub dsm_cap: u16,
    pub physical_logical_sector_size: PhysicalLogicalSectorSize, // u16
    pub inter_seek_delay: u16,
    pub world_wide_name: [u16; 4],
    _reserved_for_world_wide_name_128: [u16; 4],
    _reserved_for_tlc_technical_report: u16,
    pub words_per_logical_sector: [u16; 2],
    pub command_set_support_extension: CommandSetExtension, // u16
    pub command_set_active_extension: CommandSetExtension,  // u16
    _reserved_for_expanded_support_and_active: [u16; 6],
    pub msn_support: MsnSupport,         // u16
    pub security_status: SecurityStatus, // u16
    _reserved_word_129: [u16; 31],
    pub cfa_power_model: CfaPowerModel, // u16
    _reserved_for_cfa_word_161: [u16; 7],
    pub nominal_form_factor: NominalFormFactor, // u16
    pub data_set_management_feature: DataSetManagementFeature, // u16
    pub additional_product_id: [u16; 4],
    _reserved_for_cfa_word_174: [u16; 2],
    pub current_medial_serial_number: [u16; 30],
    pub sct_command_transport: SCTCommandTransport, // u16
    _reserved_word_207: [u16; 2],
    pub block_alignment: BlockAlignment, // u16
    pub write_read_verify_sector_count_mode_3_only: [u16; 2],
    pub write_read_verify_sector_count_mode_2_only: [u16; 2],
    pub nv_cache_capabilities: NVCacheCapabilities, // u16
    pub nv_cache_size_lsw: u16,
    pub nv_cache_size_msw: u16,
    pub nominal_media_rotation_rate: u16,
    _reserved_word_218: u16,
    pub nv_cache_options: NVCacheOptions, // u16
    pub write_read_verify_sector_count_mode: u8,
    _reserved_word_220: u8,
    _reserved_word_221: u16,
    pub transport_version: TransportVersion, // [u16; 2]
    _reserved_word_224: [u16; 6],
    pub extended_number_of_user_addressable_sectors: [u32; 2],
    pub min_blocks_per_download_microcode_mode_03: u16,
    pub max_blocks_per_download_microcode_mode_03: u16,
    _reserved_word_236: [u16; 19],
    pub signature: u8,
    pub checksum: u8,
}

#[test]
fn word_offset() {
    fn offset_of<F, T>(f: F) -> usize
    where
        F: FnOnce(&Identify) -> &T,
    {
        let ident: Identify = unsafe { core::mem::zeroed() };
        let ptr = &ident;
        let field_ptr = f(ptr);
        (field_ptr as *const _ as usize) - (ptr as *const _ as usize)
    }

    // Words are 2 bytes each, so most of these offsets are being multiplied by 2
    unsafe {
        assert_eq!(51 * 2, offset_of(|i| &i._obsolete_words_51));
        assert_eq!(62 * 2, offset_of(|i| &i._obsolete_word_62));
        assert_eq!(64 * 2 + 1, offset_of(|i| &i._reserved_byte_64));
        assert_eq!(70 * 2, offset_of(|i| &i._reserved_words_70));
        assert_eq!(91 * 2 + 1, offset_of(|i| &i._reserved_word_91));
        assert_eq!(129 * 2, offset_of(|i| &i._reserved_word_129));
        assert_eq!(161 * 2, offset_of(|i| &i._reserved_for_cfa_word_161));
        assert_eq!(174 * 2, offset_of(|i| &i._reserved_for_cfa_word_174));
    }

    assert_eq!(256 * 2, core::mem::size_of::<Identify>());
}

#[repr(transparent)]
pub struct IdentifyGeneralConfiguration(u16);

impl IdentifyGeneralConfiguration {
    /// Indicates that the response was incomplete.
    pub fn response_incomplete(&self) -> bool {
        (self.0 & 0b00000000_00000100) > 0
    }
    /// Indicates when set to 1 that the device is fixed.
    pub fn fixed_device(&self) -> bool {
        (self.0 & 0b00000000_01000000) > 0
    }
    /// Indicates when set to 1 that the media is removable.
    pub fn removable_device(&self) -> bool {
        (self.0 & 0b00000000_10000000) > 0
    }
    /// Indicates when set to 1 that the device is an ATA device.
    pub fn is_ata(&self) -> bool {
        (self.0 & 0b10000000_00000000) > 0
    }
}

impl core::fmt::Debug for IdentifyGeneralConfiguration {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("IdentifyGeneralConfiguration")
            .field("response_incomplete", &self.response_incomplete())
            .field("fixed_device", &self.fixed_device())
            .field("removable_device", &self.removable_device())
            .field("is_ata", &self.is_ata())
            .finish()
    }
}

#[repr(transparent)]
pub struct TrustedComputing(u16);

impl TrustedComputing {
    pub fn is_supported(&self) -> bool {
        (self.0 & 0b00000000_00000001) > 0
    }
}

#[repr(transparent)]
pub struct Capabilities([u16; 2]);

impl Capabilities {
    pub fn current_long_physical_sector_alignment(&self) -> u8 {
        ((self.0[0] & 0b11000000_00000000) >> 14) as u8
    }

    pub fn dma_supported(&self) -> bool {
        (self.0[0] & 0b00000000_10000000) > 0
    }

    pub fn lba48_supported(&self) -> bool {
        (self.0[0] & 0b00000100_00000000) > 0
    }

    pub fn lba_supported(&self) -> bool {
        (self.0[0] & 0b00000000_01000000) > 0
    }

    pub fn iordy_disable(&self) -> bool {
        (self.0[0] & 0b00000000_00100000) > 0
    }

    pub fn iordy_supported(&self) -> bool {
        (self.0[0] & 0b00000000_00010000) > 0
    }

    pub fn standby_timer_support(&self) -> bool {
        (self.0[0] & 0b00000000_00000100) > 0
    }
}

impl core::fmt::Debug for Capabilities {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("Capabilities")
            .field("0.0", &format_args!("{:016b}", self.0[0]))
            .field("0.1", &format_args!("{:016b}", self.0[1]))
            .field(
                "current_long_physical_sector_alignment",
                &self.current_long_physical_sector_alignment(),
            )
            .finish()
    }
}

#[repr(transparent)]
pub struct TranslationFieldsValid(u8);

impl TranslationFieldsValid {
    pub fn is_valid(&self) -> u8 {
        self.0 >> 5
    }
}

#[repr(transparent)]
pub struct NestedSettings(u8);

impl NestedSettings {
    pub fn multi_sector_setting_valid(&self) -> bool {
        (self.0 & 0b10000000) > 0
    }
    pub fn sanitize_feature_supported(&self) -> bool {
        (self.0 & 0b00001000) > 0
    }
    pub fn crypto_scramble_ext_command_supported(&self) -> bool {
        (self.0 & 0b00000100) > 0
    }
    pub fn overwrite_ext_command_supported(&self) -> bool {
        (self.0 & 0b00000010) > 0
    }
    pub fn block_erase_ext_command_supported(&self) -> bool {
        (self.0 & 0b00000001) > 0
    }
}

#[repr(transparent)]
pub struct AdditionalSupported(u16);

impl AdditionalSupported {
    pub fn zoned_capabilities(&self) -> u8 {
        (self.0 >> 14) as u8
    }
    pub fn non_volatile_write_cache(&self) -> bool {
        (self.0 & 0b00100000_00000000) > 0
    }
    pub fn extended_user_addressable_sectors_supported(&self) -> bool {
        (self.0 & 0b00010000_00000000) > 0
    }
    pub fn device_encrypts_all_user_data(&self) -> bool {
        (self.0 & 0b00001000_00000000) > 0
    }
    pub fn read_zero_after_trim_supported(&self) -> bool {
        (self.0 & 0b00000100_00000000) > 0
    }
    pub fn optional_128_bit_commands_supported(&self) -> bool {
        (self.0 & 0b00000010_00000000) > 0
    }
    pub fn ieee1667(&self) -> bool {
        (self.0 & 0b00000001_00000000) > 0
    }
    pub fn download_microcode_dma_supported(&self) -> bool {
        (self.0 & 0b00000000_10000000) > 0
    }
    pub fn set_max_set_password_unlock_dma_supported(&self) -> bool {
        (self.0 & 0b00000000_01000000) > 0
    }
    pub fn write_buffer_dma_supported(&self) -> bool {
        (self.0 & 0b00000000_00100000) > 0
    }
    pub fn read_buffer_dma_supported(&self) -> bool {
        (self.0 & 0b00000000_00010000) > 0
    }
    pub fn device_config_identify_set_dma_supported(&self) -> bool {
        (self.0 & 0b00000000_00001000) > 0
    }
    pub fn lpsaerc_supported(&self) -> bool {
        (self.0 & 0b00000000_00000100) > 0
    }
    pub fn deterministic_read_after_trim_supported(&self) -> bool {
        (self.0 & 0b00000000_00000010) > 0
    }
    pub fn cfast_spec_supported(&self) -> bool {
        (self.0 & 0b00000000_00000001) > 0
    }
}

#[repr(transparent)]
pub struct QueueDepth(u16);

impl QueueDepth {
    pub fn depth(&self) -> u16 {
        self.0 >> 11
    }
}

#[repr(transparent)]
pub struct SerialAtaCapabilities(u32);

impl SerialAtaCapabilities {
    pub fn sata_gen_1(&self) -> bool {
        (self.0 & 0b01000000_00000000_00000000_00000000) > 0
    }
    pub fn sata_gen_2(&self) -> bool {
        (self.0 & 0b00100000_00000000_00000000_00000000) > 0
    }
    pub fn sata_gen_3(&self) -> bool {
        (self.0 & 0b00010000_00000000_00000000_00000000) > 0
    }
    pub fn ncq(&self) -> bool {
        (self.0 & 0b00000000_10000000_00000000_00000000) > 0
    }
    pub fn hipm(&self) -> bool {
        (self.0 & 0b00000000_01000000_00000000_00000000) > 0
    }
    pub fn phy_events(&self) -> bool {
        (self.0 & 0b00000000_00100000_00000000_00000000) > 0
    }
    pub fn ncq_unload(&self) -> bool {
        (self.0 & 0b00000000_00010000_00000000_00000000) > 0
    }
    pub fn ncq_priority(&self) -> bool {
        (self.0 & 0b00000000_00001000_00000000_00000000) > 0
    }
    pub fn host_auto_ps(&self) -> bool {
        (self.0 & 0b00000000_00000100_00000000_00000000) > 0
    }
    pub fn device_auto_ps(&self) -> bool {
        (self.0 & 0b00000000_00000010_00000000_00000000) > 0
    }
    pub fn read_log_dma(&self) -> bool {
        (self.0 & 0b00000000_00000001_00000000_00000000) > 0
    }
    pub fn current_speed(&self) -> u8 {
        ((self.0 & 0b00000000_00000000_01110000_00000000) >> 12) as u8
    }
    pub fn ncq_streaming(&self) -> bool {
        (self.0 & 0b00000000_00000000_00001000_00000000) > 0
    }
    pub fn ncq_queue_management(&self) -> bool {
        (self.0 & 0b00000000_00000000_00000100_00000000) > 0
    }
    pub fn ncq_receive_send(&self) -> bool {
        (self.0 & 0b00000000_00000000_00000010_00000000) > 0
    }
    pub fn devslp_to_reduced_power_state(&self) -> bool {
        (self.0 & 0b00000000_00000000_00000001_00000000) > 0
    }
}

#[repr(transparent)]
pub struct SerialAtaFeatures(u16);
impl SerialAtaFeatures {
    pub fn non_zero_offsets(&self) -> bool {
        (self.0 & 0b01000000_00000000) > 0
    }
    pub fn dma_setup_auto_activate(&self) -> bool {
        (self.0 & 0b00100000_00000000) > 0
    }
    pub fn dipm(&self) -> bool {
        (self.0 & 0b00010000_00000000) > 0
    }
    pub fn in_order_data(&self) -> bool {
        (self.0 & 0b00001000_00000000) > 0
    }
    pub fn hardware_feature_control(&self) -> bool {
        (self.0 & 0b00000100_00000000) > 0
    }
    pub fn software_settings_preservation(&self) -> bool {
        (self.0 & 0b00000010_00000000) > 0
    }
    pub fn ncq_autosense(&self) -> bool {
        (self.0 & 0b00000001_00000000) > 0
    }
    pub fn devslp(&self) -> bool {
        (self.0 & 0b00000000_10000000) > 0
    }
    pub fn hybrid_information(&self) -> bool {
        (self.0 & 0b00000000_01000000) > 0
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct CommandSet([u8; 6]);

impl CommandSet {
    /*
      USHORT SmartCommands : 1;
      USHORT SecurityMode : 1;
      USHORT RemovableMediaFeature : 1;
      USHORT PowerManagement : 1;
      USHORT Reserved1 : 1;
      USHORT WriteCache : 1;
      USHORT LookAhead : 1;
      USHORT ReleaseInterrupt : 1;

      USHORT ServiceInterrupt : 1;
      USHORT DeviceReset : 1;
      USHORT HostProtectedArea : 1;
      USHORT Obsolete1 : 1;
      USHORT WriteBuffer : 1;
      USHORT ReadBuffer : 1;
      USHORT Nop : 1;
      USHORT Obsolete2 : 1;

      USHORT DownloadMicrocode : 1;
      USHORT DmaQueued : 1;
      USHORT Cfa : 1;
      USHORT AdvancedPm : 1;
      USHORT Msn : 1;
      USHORT PowerUpInStandby : 1;
      USHORT ManualPowerUp : 1;
      USHORT Reserved2 : 1;

      USHORT SetMax : 1;
      USHORT Acoustics : 1;
      USHORT BigLba : 1;
      USHORT DeviceConfigOverlay : 1;
      USHORT FlushCache : 1;
      USHORT FlushCacheExt : 1;
      USHORT WordValid83 : 2;

      USHORT SmartErrorLog : 1;
      USHORT SmartSelfTest : 1;
      USHORT MediaSerialNumber : 1;
      USHORT MediaCardPassThrough : 1;
      USHORT StreamingFeature : 1;
      USHORT GpLogging : 1;
      USHORT WriteFua : 1;
      USHORT WriteQueuedFua : 1;

      USHORT WWN64Bit : 1;
      USHORT URGReadStream : 1;
      USHORT URGWriteStream : 1;
      USHORT ReservedForTechReport : 2;
      USHORT IdleWithUnloadFeature : 1;
      USHORT WordValid : 2;
    */
}

#[repr(transparent)]
pub struct EraseUnit(u16);

impl EraseUnit {
    /*
    struct {
      USHORT TimeRequired : 15;
      USHORT ExtendedTimeReported : 1;
    }
    */
}

#[repr(transparent)]
pub struct PhysicalLogicalSectorSize(u16);

impl PhysicalLogicalSectorSize {
    /*
    struct {
      USHORT LogicalSectorsPerPhysicalSector : 4;
      USHORT Reserved0 : 8;
      USHORT LogicalSectorLongerThan256Words : 1;
      USHORT MultipleLogicalSectorsPerPhysicalSector : 1;
      USHORT Reserved1 : 2;
    } PhysicalLogicalSectorSize;
    */
}

#[repr(transparent)]
pub struct CommandSetExtension(u16);

impl CommandSetExtension {
    /*
    struct {
        USHORT ReservedForDrqTechnicalReport : 1;
        USHORT WriteReadVerify : 1;
        USHORT WriteUncorrectableExt : 1;
        USHORT ReadWriteLogDmaExt : 1;
        USHORT DownloadMicrocodeMode3 : 1;
        USHORT FreefallControl : 1;
        USHORT SenseDataReporting : 1;
        USHORT ExtendedPowerConditions : 1;
        USHORT Reserved0 : 6;
        USHORT WordValid : 2;
    }
    */
}
#[repr(transparent)]
pub struct MsnSupport(u16);

impl MsnSupport {
    pub fn support(&self) -> u8 {
        (self.0 >> 14) as u8
    }
}

#[repr(transparent)]
pub struct SecurityStatus(u16);

impl SecurityStatus {
    /*
    struct {
      USHORT SecuritySupported : 1;
      USHORT SecurityEnabled : 1;
      USHORT SecurityLocked : 1;
      USHORT SecurityFrozen : 1;
      USHORT SecurityCountExpired : 1;
      USHORT EnhancedSecurityEraseSupported : 1;
      USHORT Reserved0 : 2;
      USHORT SecurityLevel : 1;
      USHORT Reserved1 : 7;
    } SecurityStatus;

      */
}

#[repr(transparent)]
pub struct CfaPowerModel(u16);

impl CfaPowerModel {
    /*
    struct {
      USHORT MaximumCurrentInMA : 12;
      USHORT CfaPowerMode1Disabled : 1;
      USHORT CfaPowerMode1Required : 1;
      USHORT Reserved0 : 1;
      USHORT Word160Supported : 1;
    } CfaPowerMode1;
      */
}

#[repr(transparent)]
pub struct NominalFormFactor(u16);

impl NominalFormFactor {
    pub fn get(&self) -> u8 {
        (self.0 >> 12) as u8
    }
}

#[repr(transparent)]
pub struct DataSetManagementFeature(u16);

impl DataSetManagementFeature {
    pub fn supports_trim(&self) -> bool {
        (self.0 & 0b10000000_00000000) > 0
    }
}

#[repr(transparent)]
pub struct SCTCommandTransport(u16);

impl SCTCommandTransport {
    /*
    struct {
    USHORT Supported : 1;
    USHORT Reserved0 : 1;
    USHORT WriteSameSuported : 1;
    USHORT ErrorRecoveryControlSupported : 1;
    USHORT FeatureControlSuported : 1;
    USHORT DataTablesSuported : 1;
    USHORT Reserved1 : 6;
    USHORT VendorSpecific : 4;
    } SCTCommandTransport;
    */
}

#[repr(transparent)]
pub struct BlockAlignment(u16);

impl BlockAlignment {
    /*
    struct {
    USHORT AlignmentOfLogicalWithinPhysical : 14;
    USHORT Word209Supported : 1;
    USHORT Reserved0 : 1;
    } BlockAlignment;
    */
}

#[repr(transparent)]
pub struct NVCacheCapabilities(u16);

impl NVCacheCapabilities {
    /*
      struct {
        USHORT NVCachePowerModeEnabled : 1;
        USHORT Reserved0 : 3;
        USHORT NVCacheFeatureSetEnabled : 1;
        USHORT Reserved1 : 3;
        USHORT NVCachePowerModeVersion : 4;
        USHORT NVCacheFeatureSetVersion : 4;
      } NVCacheCapabilities;
    */
}

#[repr(transparent)]
pub struct NVCacheOptions(u16);

impl NVCacheOptions {
    /*
    struct {
        UCHAR NVCacheEstimatedTimeToSpinUpInSeconds;
        UCHAR Reserved;
    } NVCacheOptions;
    */
}

#[repr(transparent)]
pub struct TransportVersion([u16; 2]);

impl TransportVersion {
    pub fn major_version(&self) -> u16 {
        self.0[0] >> 4
    }

    pub fn transport_type(&self) -> u8 {
        (self.0[0] & 0b00001111) as u8
    }

    pub fn minor_version(&self) -> u16 {
        self.0[1]
    }
}
