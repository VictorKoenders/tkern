mod guid;
mod list;

mod prelude {
    pub use uefi::proto::console::text::{Key, ScanCode};
    pub use uefi_services::{print, println};
}

use self::list::ListOptions;
use crate::view::guid::GuidType;
use alloc::vec::Vec;
use uefi::table::boot::SearchType;
use uefi_services::{print, println};

type TableTy = uefi::table::SystemTable<uefi::table::Boot>;

pub fn run(table: &mut TableTy) {
    list::list(
        table,
        ListOptions {
            entries: &["Config", "Protocols"],
            render: |_: &mut _, t: &_| {
                println!("{t}");
            },
            select: |table: &mut _, t: &_| match t {
                &"Config" => render_config(table),
                &"Protocols" => render_protocols(table),
                _ => {}
            },
        },
    )
}

fn render_config(table: &mut TableTy) {
    let mut entries: Vec<_> = table
        .config_table()
        .iter()
        .map(|e| (e.address, e.guid))
        .collect();
    entries.sort_by_key(|e| e.0);
    list::list(
        table,
        ListOptions {
            entries: &entries,
            render: |_: &mut _, (address, guid): &_| match guid::GuidType::from_guid(*guid) {
                GuidType::Config(c) => println!("Config: {c:?} {:p}", address),
                GuidType::Protocol(c) => println!("Protocol: {c:?} {:p}", address),
                GuidType::Unknown(c) => println!("Unknown: {c} {:p}", address),
            },
            select: |_: &mut _, _: &_| {},
        },
    );
}

fn render_protocols(table: &mut TableTy) {
    let protocols = guid::GuidProtocol::all();
    list::list(
        table,
        ListOptions {
            entries: protocols,
            render: |table: &mut TableTy, t: &guid::GuidProtocol| {
                let count = table
                    .boot_services()
                    .locate_handle(SearchType::ByProtocol(&t.guid()), None)
                    .unwrap_or(0);
                println!("{t:?} ({count} handles)");
            },
            select: |_: &mut _, t: &_| {},
        },
    )
}
