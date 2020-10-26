use super::{
    bus::{Bus, Command, Features, Status},
    Error,
};

pub static INNER: spin::Mutex<Inner> = spin::Mutex::new(Inner::new());
const SECTOR_SIZE: u16 = 2048;

pub struct Inner {
    drive: u8,
}

impl Inner {
    const fn new() -> Self {
        Self {
            // Set to 255 by default, so the first select will make sure to select the actual drive
            // This assumes we'll never have 256 drives in the system
            drive: 255,
        }
    }

    pub fn drive_select(&mut self, bus: Bus, drive: u8) -> Result<(), Error> {
        if self.drive == drive {
            bus.status().as_err(bus)?;
            return Ok(());
        }
        bus.set_drive(drive);
        if self.select_delay(bus).contains(Status::ERROR) {
            return Err(Error::BusSelect);
        }

        bus.features(Features::NONE); // PIO mode
        bus.set_lba_mid((SECTOR_SIZE & 0xFF) as u8);
        bus.set_lba_high((SECTOR_SIZE >> 8) as u8);
        bus.command(Command::ATA_PACKET);

        self.busy_loop(bus).as_err(bus)?;
        self.drive = drive;
        Ok(())
    }

    fn select_delay(&self, bus: Bus) -> Status {
        // need to sleep for 400 ns
        // each status read is roughly 100ns, so we read 5 times, and return the last status
        let mut i = 0;
        loop {
            let status = bus.status();
            i += 1;
            if i == 5 {
                break status;
            }
        }
    }

    pub fn busy_loop(&self, bus: Bus) -> Status {
        // first we loop on BUSY
        while bus.status().contains(Status::BUSY) {
            core::sync::atomic::spin_loop_hint();
        }

        // Then we loop on BUSY and !ERR
        loop {
            let status = bus.status();
            if !status.contains(Status::BUSY) || status.contains(Status::ERROR) {
                return status;
            }
            core::sync::atomic::spin_loop_hint();
        }
    }
}
