use std::{env, error::Error};
use crate::{config, modules};

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") { println!("rfetch 0.1.0\nUsage: rfetch [--logo NAME] [--no-color] [--gen-config]\n\nOptions: --logo NAME  Select logo (auto or a built-in name)\n         --no-color   Disable ANSI colors\n         --gen-config Write default config and exit\n         --version    Show version"); return Ok(()); }
    if args.iter().any(|a| a == "--version" || a == "-V") { println!("rfetch {}", env!("CARGO_PKG_VERSION")); return Ok(()); }
    let mut logo_arg = None;
    let mut no_color = false;
    let mut generate_config = false;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--no-color" => no_color = true,
            "--gen-config" => generate_config = true,
            "--logo" => logo_arg = Some(it.next().ok_or("--logo requires a value")?.clone()),
            x if x.starts_with('-') => return Err(format!("unknown option: {x}").into()),
            x => return Err(format!("unexpected argument: {x}").into()),
        }
    }
    let mut config = config::load()?;
    if generate_config { config::write_default()?; println!("Wrote {}", config::path()?.display()); return Ok(()); }
    if let Some(value) = logo_arg { config.logo.kind = value; }
    if no_color { config.general.color = false; }
    modules::render(&config)?;
    Ok(())
}
