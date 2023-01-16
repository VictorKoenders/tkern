use clap::{builder::PossibleValue, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // TODO: `Docs` command
    // TODO: `Fmt` command
    Run(RunArgs),
    Build(BuildArgs),
    ObjDump(ObjDumpArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    pub arch: Arch,
    pub emulator: Emulator,
    #[arg(long, short)]
    pub release: bool,
}
#[derive(Args, Debug)]
pub struct BuildArgs {
    pub arch: Arch,
    #[arg(long, short)]
    pub with_qemu: bool,
    #[arg(long, short)]
    pub release: bool,
}
#[derive(Args, Debug)]
pub struct ObjDumpArgs {
    pub arch: Arch,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Arch {
    X86_64,
    Aarch64,
}

impl ValueEnum for Arch {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::X86_64, Self::Aarch64]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Aarch64 => PossibleValue::new("aarch64"),
            Self::X86_64 => PossibleValue::new("x86_64"),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Emulator {
    Qemu,
}
