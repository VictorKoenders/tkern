use std::path::PathBuf;

use cargo::core::Workspace;

mod cli_args;
mod commands;

mod prelude;

fn main() {
    let cli: cli_args::Cli = clap::Parser::parse();

    let project_root = project_root();
    let workspace_config = workspace_config();
    let mut workspace =
        Workspace::new(&project_root, &workspace_config).expect("Could not load workspace");

    let result = match cli.command {
        cli_args::Commands::Run(args) => commands::run(&mut workspace, args),
        cli_args::Commands::Build(args) => commands::build(&mut workspace, args).map(|_| ()),
        cli_args::Commands::ObjDump(args) => commands::obj_dump(&mut workspace, args),
    };

    if let Err(e) = result {
        eprintln!("{e:?}");
    }
}

fn workspace_config() -> cargo::Config {
    let mut config = cargo::Config::default().unwrap();
    config.nightly_features_allowed = true;
    config
}

fn project_root() -> PathBuf {
    let mut dir = std::env::current_dir().expect("Could not get current dir");
    loop {
        if dir.join("kernel/Cargo.toml").exists() {
            return dir.join("kernel/Cargo.toml");
        }
        dir = dir
            .parent()
            .expect("Could not find project root")
            .to_path_buf();
    }
}
