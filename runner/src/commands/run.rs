// TODO:
// - prompt if OVM-pure-efi is not installed, to download it
// - prompt if qemu-system-x86_64 is not installed, to download it

use std::{path::PathBuf, process::Command};

use crate::cli_args::*;
use cargo::core::Workspace;

pub fn run(workspace: &mut Workspace, args: RunArgs) -> std::io::Result<()> {
    let path = super::build::build(
        workspace,
        BuildArgs {
            arch: args.arch,
            with_qemu: true,
            release: args.release,
        },
    )?;
    match args.arch {
        Arch::X86_64 => run_x86_64(args, path),
        Arch::Aarch64 => unimplemented!(),
    }
}

fn run_x86_64(_args: RunArgs, path: PathBuf) -> std::io::Result<()> {
    let dir = path.parent().unwrap();

    let ovmf_path = dir.join("OVMF-pure-efi.fd");
    if !ovmf_path.exists() {
        println!("Downloading OVMF uefi bios");
        let bytes = reqwest::blocking::get("https://github.com/rust-osdev/ovmf-prebuilt/releases/download/v0.20220719.209%2Bgf0064ac3af/OVMF-pure-efi.fd").unwrap().bytes().unwrap();
        std::fs::write(ovmf_path, bytes).unwrap();
    }

    let mut command = Command::new("qemu-system-x86_64");
    let drive = format!(
        "format=raw,file={}",
        path.with_extension("gdt")
            .file_name()
            .unwrap()
            .to_string_lossy()
    );
    #[rustfmt::skip]
    command.current_dir(dir).args([
        "-drive", &drive,
        "-bios", "OVMF-pure-efi.fd",
        "-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"
    ]);
    debug_cmd(&command);
    let output = command.spawn().unwrap().wait().unwrap();

    if !output.success() && output.code().is_some() {
        std::process::exit(output.code().unwrap());
    }
    Ok(())
}

fn debug_cmd(command: &Command) {
    print!("Running: {}", command.get_program().to_string_lossy());
    for arg in command.get_args() {
        print!(" {arg:?}");
    }
    println!();
    if let Some(dir) = command.get_current_dir() {
        println!("  in dir {}", dir.to_string_lossy());
    }
}
