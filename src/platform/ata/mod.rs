// Most of the information in the ata driver comes from:
// - https://wiki.osdev.org/ATAPI
// - https://wiki.osdev.org/ATA_PIO_Mode
// I highly recommend you skim through those article before reading the code in this file.

mod bus;
mod identity;

use self::{bus::*, identity::Identity};

#[derive(Debug)]
pub enum Error {
    NoDeviceConnected,
    DriveDoesNotExist,
    DriveNotAta,
    AtapiOrSata,
    CouldNotIdentify,
    IdentifyTimeout,
}

pub struct BusState {
    bus: &'static Bus,
    current_drive: u8,
    pub identity: Identity,
}

impl BusState {
    fn init_primary() -> Result<BusState, Error> {
        let bus = Bus::PRIMARY;
        // If there is no disk connected to the bus at all, then the electrical values on the bus will all go "high" (to +5 volts).
        // A computer will read this as an 0xFF byte -- this is a condition called a "floating" bus.
        // This is an excellent way to find out if there are no drives on a bus
        if bus.get_raw_status() == 0xFF {
            return Err(Error::NoDeviceConnected);
        }
        let signature = [
            bus.get_sector_count(),
            bus.get_lba_low(),
            bus.get_lba_mid(),
            bus.get_lba_high(),
        ];

        // If these contain 0x01, 0x01, 0x14, 0xEB then the device is a packet device, and `IDENTIFY PACKET DEVICE` (0xA1) should be used.
        if signature == [0x01, 0x01, 0x14, 0xEB] {
            unimplemented!("ATA IDENTIFY PACKET DEVICE (0xA1)");
        }

        // otherwise the connected device is a non-packet device, and `IDENTIFY DEVICE` (0xEC) should work
        let mut state = BusState {
            bus,
            current_drive: 0x0,
            identity: Identity::default(),
        };
        // To use the IDENTIFY command, select a target drive by sending 0xA0 for the master drive, or 0xB0 for the slave, to the "drive select" IO port.
        state.identify(0xa0)?;
        Ok(state)
    }

    fn identify(&mut self, drive: u8) -> Result<(), Error> {
        // for more info see https://wiki.osdev.org/ATA_PIO_Mode#IDENTIFY_command

        // To use the IDENTIFY command, select a target drive by sending 0xA0 for the master drive, or 0xB0 for the slave, to the "drive select" IO port.
        self.bus.set_drive(drive);
        // Then set the Sectorcount, LBAlo, LBAmid, and LBAhi IO ports to 0
        self.bus.set_sector_count(0);
        self.bus.set_lba_low(0);
        self.bus.set_lba_mid(0);
        self.bus.set_lba_high(0);
        // Then send the IDENTIFY command to the Command IO port.
        self.bus.send_command(Command::Identify);

        // Then read the Status port
        let mut status = self.bus.get_status();
        // If the value read is 0, the drive does not exist.
        if status.is_empty() {
            return Err(Error::DriveDoesNotExist);
        }

        if status.contains(Status::ERROR) {
            // ATAPI or SATA devices are supposed to respond to an ATA IDENTIFY command by immediately reporting an error in the Status Register
            return Err(Error::AtapiOrSata);
        }

        // For any other value: poll the Status port until BUSY clears.
        while status.contains(Status::BUSY) {
            status = self.bus.get_status();
        }

        // Because of some ATAPI drives that do not follow spec, at this point you need to check the LBAmid and LBAhi ports (0x1F4 and 0x1F5) to see if they are non-zero.
        let mid = self.bus.get_lba_mid();
        let high = self.bus.get_lba_high();
        // If they are non-zero, the drive is not ATA, and you should stop polling
        if mid > 0 && high > 0 {
            return Err(Error::DriveNotAta);
        }

        // continue polling one of the Status ports until DRQ or ERR sets.
        let mut count = 0;
        while !status.contains(Status::ERROR) && status.contains(Status::DRQ) {
            crate::platform::halt();
            status = self.bus.get_status();
            count += 1;
            if count == 100 {
                return Err(Error::IdentifyTimeout);
            }
        }

        if status.contains(Status::ERROR) {
            return Err(Error::CouldNotIdentify);
        }

        // the data is ready to read from the Data port.
        self.identity.fill(self.bus);

        Ok(())
    }

    pub fn status(&self) -> Status {
        self.bus.get_status()
    }

    fn drive_select(&self, drive: u8) -> Result<(), ()> {
        if self.current_drive == drive {
            return Ok(());
        }
        self.bus.set_drive(drive);
        Ok(())
    }
}

pub struct Ata {
    pub primary: BusState,
}

pub fn init() -> Result<Ata, Error> {
    Ok(Ata {
        primary: BusState::init_primary()?,
    })
}
