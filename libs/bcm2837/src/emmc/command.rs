#![allow(deprecated)]

use bitflags::bitflags;
use core::{fmt, num::NonZeroU16};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
#[non_exhaustive]
pub enum Command {
    GoIdleState,
    AllSendCid,
    SendRelAddr,
    SetDsr,
    SwitchFunc,
    CardSelect,
    SendIfCond,
    SendCsd,
    SendCid,
    VoltageSwitch,
    StopTrans,
    SendStatus,
    GoInactive,
    SetBlocklen,
    ReadSingle,
    ReadMulti,
    SendTuning,
    SpeedClass,
    SetBlockcnt,
    WriteSingle,
    WriteMulti,
    ProgramCsd,
    SetWritePr,
    ClrWritePr,
    SndWritePr,
    EraseWrSt,
    EraseWrEnd,
    ERASE,
    LockUnlock,
    AppCmd,
    #[deprecated(note = "Duplicate of AppCmd?")]
    AppCmdRca,
    GenCmd,
    #[deprecated(note = "Not implemented")]
    AppCmdStart,
    #[deprecated(note = "Duplicate of SwitchFunc")]
    SetBusWidth,
    #[deprecated(note = "Duplicate of SendStatus")]
    EmmcStatus,
    SendNumWrbl,
    #[deprecated(note = "Duplicate of SetBlockcnt")]
    SendNumErs,
    AppSendOpCond,
    #[deprecated(note = "Duplicate of LockUnlock")]
    SetClrDet,
    SendScr,
}

impl fmt::Debug for Command {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Command")
            .field("variant", &self.name())
            .field("use_rca", &self.use_rca())
            .field("delay", &self.delay())
            .finish_non_exhaustive()
    }
}

impl fmt::Display for Command {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.name())
    }
}

impl Command {
    pub const fn name(self) -> &'static str {
        match self {
            Command::GoIdleState => "GO_IDLE_STATE",
            Command::AllSendCid => "ALL_SEND_CID",
            Command::SendRelAddr => "SEND_REL_ADDR",
            Command::SetDsr => "SET_DSR",
            Command::SwitchFunc => "SWITCH_FUNC",
            Command::CardSelect => "CARD_SELECT",
            Command::SendIfCond => "SEND_IF_COND",
            Command::SendCsd => "SEND_CSD",
            Command::SendCid => "SEND_CID",
            Command::VoltageSwitch => "VOLTAGE_SWITCH",
            Command::StopTrans => "STOP_TRANS",
            Command::SendStatus => "SEND_STATUS",
            Command::GoInactive => "GO_INACTIVE",
            Command::SetBlocklen => "SET_BLOCK_LEN",
            Command::ReadSingle => "READ_SINGLE",
            Command::ReadMulti => "READ_MULTI",
            Command::SendTuning => "SEND_TUNING",
            Command::SpeedClass => "SPEED_CLASS",
            Command::SetBlockcnt => "SET_BLOCKCNT",
            Command::WriteSingle => "WRITE_SINGLE",
            Command::WriteMulti => "WRITE_MULTI",
            Command::ProgramCsd => "PROGRAM_CSD",
            Command::SetWritePr => "SET_WRITE_PR",
            Command::ClrWritePr => "CLR_WRITE_PR",
            Command::SndWritePr => "SND_WRITE_PR",
            Command::EraseWrSt => "ERASE_WR_ST",
            Command::EraseWrEnd => "ERASE_WR_END",
            Command::ERASE => "ERASE",
            Command::LockUnlock => "LOCK_UNLOCK",
            Command::AppCmd => "APP_CMD",
            Command::AppCmdRca => "APP_CMD_RCA",
            Command::GenCmd => "GEN_CMD",
            Command::AppCmdStart => "APP_CMD_START",
            Command::SetBusWidth => "SET_BUS_WIDTH",
            Command::EmmcStatus => "EMMC_STATUS",
            Command::SendNumWrbl => "SEND_NUM_WRBL",
            Command::SendNumErs => "SEND_NUM_ERS",
            Command::AppSendOpCond => "APP_SEND_OP_COND",
            Command::SetClrDet => "SET_CLR_DET",
            Command::SendScr => "SEND_SCR",
        }
    }

    pub const fn use_rca(self) -> CommandRca {
        match self {
            Self::CardSelect
            | Self::SendIfCond
            | Self::SendCsd
            | Self::SendCid
            | Self::SendStatus
            | Self::GoInactive
            | Self::AppCmdRca
            | Self::EmmcStatus => CommandRca::RCA,
            _ => CommandRca::empty(),
        }
    }

    pub const fn delay(self) -> Option<NonZeroU16> {
        match self {
            Self::SendIfCond | Self::AppCmd => NonZeroU16::new(100),
            Self::AppSendOpCond => NonZeroU16::new(1000),
            _ => None,
        }
    }

    pub const fn requires_app_cmd(self) -> bool {
        matches!(
            self,
            Self::AppCmdStart
                | Self::SetBusWidth
                | Self::EmmcStatus
                | Self::SendNumWrbl
                | Self::SendNumErs
                | Self::AppSendOpCond
                | Self::SetClrDet
                | Self::SendScr
        )
    }

    pub const fn response_size(self) -> ResponseSize {
        match self {
            Command::GoIdleState => ResponseSize::None,
            Command::AllSendCid => ResponseSize::_136Bits,
            Command::SendRelAddr => ResponseSize::_48Bits,
            Command::SetDsr => ResponseSize::None,
            Command::SwitchFunc => ResponseSize::_48Bits,
            Command::CardSelect => ResponseSize::_48BitsWithBusy,
            Command::SendIfCond => ResponseSize::_48Bits,
            Command::SendCsd => ResponseSize::_136Bits,
            Command::SendCid => ResponseSize::_136Bits,
            Command::VoltageSwitch => ResponseSize::_48Bits,
            Command::StopTrans => ResponseSize::_48BitsWithBusy,
            Command::SendStatus => ResponseSize::_48Bits,
            Command::GoInactive => ResponseSize::None,
            Command::SetBlocklen => ResponseSize::None,
            Command::ReadSingle => ResponseSize::_48Bits,
            Command::ReadMulti => ResponseSize::_48Bits,
            Command::SendTuning => ResponseSize::_48Bits,
            Command::SpeedClass => ResponseSize::_48BitsWithBusy,
            Command::SetBlockcnt => ResponseSize::_48Bits,
            Command::WriteSingle => ResponseSize::_48Bits,
            Command::WriteMulti => ResponseSize::_48Bits,
            Command::ProgramCsd => ResponseSize::_48Bits,
            Command::SetWritePr => ResponseSize::_48BitsWithBusy,
            Command::ClrWritePr => ResponseSize::_48BitsWithBusy,
            Command::SndWritePr => ResponseSize::_48Bits,
            Command::EraseWrSt => ResponseSize::_48Bits,
            Command::EraseWrEnd => ResponseSize::_48Bits,
            Command::ERASE => ResponseSize::_48BitsWithBusy,
            Command::LockUnlock => ResponseSize::_48Bits,
            Command::AppCmd => ResponseSize::None,
            Command::AppCmdRca => ResponseSize::_48Bits,
            Command::GenCmd => ResponseSize::_48Bits,
            Command::AppCmdStart => unimplemented!(),
            Command::SetBusWidth => ResponseSize::_48Bits,
            Command::EmmcStatus => ResponseSize::_48Bits,
            Command::SendNumWrbl => ResponseSize::_48Bits,
            Command::SendNumErs => ResponseSize::_48Bits,
            Command::AppSendOpCond => ResponseSize::_48Bits,
            Command::SetClrDet => ResponseSize::_48Bits,
            Command::SendScr => ResponseSize::_48Bits,
        }
    }

    pub const fn code(&self) -> u8 {
        match self {
            Command::GoIdleState => 0x00,
            Command::AllSendCid => 0x02,
            Command::SendRelAddr => 0x03,
            Command::SetDsr => 0x04,
            Command::SwitchFunc => 0x06,
            Command::CardSelect => 0x07,
            Command::SendIfCond => 0x08,
            Command::SendCsd => 0x09,
            Command::SendCid => 0x0a,
            Command::VoltageSwitch => 0x0b,
            Command::StopTrans => 0x0c,
            Command::SendStatus => 0x0d,
            Command::GoInactive => 0x0f,
            Command::SetBlocklen => 0x10,
            Command::ReadSingle => 0x11,
            Command::ReadMulti => 0x12,
            Command::SendTuning => 0x13,
            Command::SpeedClass => 0x14,
            Command::SetBlockcnt => 0x17,
            Command::WriteSingle => 0x18,
            Command::WriteMulti => 0x19,
            Command::ProgramCsd => 0x1b,
            Command::SetWritePr => 0x1c,
            Command::ClrWritePr => 0x1d,
            Command::SndWritePr => 0x1e,
            Command::EraseWrSt => 0x20,
            Command::EraseWrEnd => 0x21,
            Command::ERASE => 0x26,
            Command::LockUnlock => 0x2a,
            Command::AppCmd => 0x37,
            Command::AppCmdRca => 0x37,
            Command::GenCmd => 0x38,
            Command::AppCmdStart => unimplemented!(),
            Command::SetBusWidth => 0x06,
            Command::EmmcStatus => 0x0d,
            Command::SendNumWrbl => 0x16,
            Command::SendNumErs => 0x17,
            Command::AppSendOpCond => 0x29,
            Command::SetClrDet => 0x2a,
            Command::SendScr => 0x33,
        }
    }

    pub fn write(self, w: &mut bcm2837_pac::emmc::cmdtm::W) -> &mut bcm2837_pac::emmc::cmdtm::W {
        let cmd = self.code();
        debug_assert!(cmd <= 0b11_1111);
        let w = unsafe { w.cmd_index().bits(cmd) };
        let w = self.response_size().write(w.cmd_rspns_type());

        match self {
            Command::ReadSingle => w.cmd_isdata().set_bit().tm_dat_dir().set_bit(),
            Command::ReadMulti => w
                .cmd_isdata()
                .set_bit()
                .tm_dat_dir()
                .set_bit()
                .tm_blkcnt_en()
                .set_bit()
                .tm_multi_block()
                .set_bit(),
            Command::WriteSingle => w.cmd_isdata().set_bit(),
            Command::WriteMulti => w
                .cmd_isdata()
                .set_bit()
                .tm_blkcnt_en()
                .set_bit()
                .tm_multi_block()
                .set_bit(),
            Command::SendScr => w.cmd_isdata().set_bit().tm_dat_dir().set_bit(),
            _ => w,
        };
        w
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResponseSize {
    None,
    _48Bits,
    _48BitsWithBusy,
    _136Bits,
}

impl ResponseSize {
    pub(crate) fn write(
        self,
        rspns: bcm2837_pac::emmc::cmdtm::CMD_RSPNS_TYPE_W,
    ) -> &mut bcm2837_pac::emmc::cmdtm::W {
        match self {
            ResponseSize::None => rspns.none(),
            ResponseSize::_48Bits => rspns._48bits(),
            ResponseSize::_48BitsWithBusy => rspns._48bits_using_busy(),
            ResponseSize::_136Bits => rspns._136bits(),
        }
    }
}

bitflags! {
    pub struct CommandRca: u8 {
        const RCA = 0b0000_0001;
    }
}
