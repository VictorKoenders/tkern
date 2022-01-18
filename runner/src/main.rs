use std::{path::PathBuf, process::Command};
use structopt::StructOpt;

mod kernel;

pub fn gcc_aarch_path() -> String {
    std::env::var("GCC_AARCH_PATH").unwrap_or_else(|_| String::from("C:/tools/gcc-arm-aarch64-none-elf"))
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Kernel runner")]
pub enum Args {
    Build {
        #[structopt(short, long)]
        release: bool,
    },
    RunQemu {
        #[structopt(short, long)]
        release: bool,
    },
    Objdump {
        #[structopt(long)]
        gcc_aarch_path: Option<PathBuf>,
    },
    Clean,
}

pub fn main() {
    match Args::from_args() {
        Args::Build {
            release
        } => {
            let out = kernel::build(release);
            kernel::objcopy(&out, "target/kernel.img");
        },
        Args::RunQemu {
            release
        } => {
            let out = kernel::build(release);
            kernel::objcopy(&out, "target/kernel.img");
            run_qemu("target/kernel.img");
        },
        Args::Clean => {
            let mut cmd = Command::new("cargo");
            cmd.arg("clean");
            cmd.current_dir(kernel::find_dir());
            utils::run(cmd)
        },
        Args::Objdump { gcc_aarch_path} => {
            objdump(&gcc_aarch_path);
        }
    }
}

pub fn run_qemu(kernel_path: &str) {
    let mut cmd = Command::new("qemu-system-aarch64");
    cmd.args(&["-M", "raspi3", "-serial", "stdio", "-display", "none", "-kernel", kernel_path]);
    cmd.current_dir(kernel::find_dir());
    utils::run(cmd);
}

mod utils {
    use std::process::Command;

    pub fn run(mut cmd: Command) {
        println!("{:?}", cmd);

        let result = cmd.spawn().unwrap().wait().unwrap();
        println!("{}", result);
        if !result.success() {
            std::process::exit(result.code().unwrap());
        }
    }
}

fn objdump(gcc_aarch_path: &Option<PathBuf>)  {
    let mut path = if let Some(path) = gcc_aarch_path.clone() {
        path
    } else if let Ok(path) = std::env::var("GCC_AARCH_PATH") {
        PathBuf::from(path)
    } else {
        PathBuf::new()
    };
    path.push("aarch64-none-elf-objdump");
    let kernel_path = kernel::build(false);
    let mut cmd = Command::new(path);
    cmd
        .args(&[
            "--disassemble",
            "--demangle",
            "--section", ".text",
            "--section", ".rodata",
            "--section", ".got",
            &kernel_path
        ]);
    cmd.current_dir(kernel::find_dir());
    utils::run(cmd);
}
