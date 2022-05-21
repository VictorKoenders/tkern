use std::{env, io::Write as _, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rustc-link-arg=-Tlinker.ld");

    let max_memory = parse_env_bytesize("TKERN_MAX_MEMORY");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut config = std::fs::File::create(dest_path).expect("Could not create config file");

    writeln!(
        &mut config,
        "pub const MAX_MEMORY: u32 = {}; // {}",
        max_memory.0, max_memory.1
    )
    .unwrap();
}

fn parse_env_bytesize(env_key: &str) -> (u32, String) {
    let var =
        std::env::var(env_key).unwrap_or_else(|e| panic!("Missing variable {}: {:?}", env_key, e));
    if let Some(non_int_index) = var
        .bytes()
        .position(|b| b != b'.' && !(b as char).is_ascii_digit())
    {
        let int_part: f32 = var[..non_int_index]
            .parse()
            .unwrap_or_else(|e| panic!("Invalid integer {:?}: {:?}", &var[..non_int_index], e));
        let size = match var[non_int_index..].to_ascii_lowercase().as_str() {
            "gb" => 1024 * 1024 * 1024,
            "mb" => 1024 * 1024,
            "b" => 1024,
            x => panic!("Unknown size: {:?}", x),
        };
        ((int_part * (size as f32)) as u32, var)
    } else if let Ok(size) = var.parse() {
        (size, var)
    } else {
        panic!("Unknown size for {}: {:?}", env_key, var);
    }
}
