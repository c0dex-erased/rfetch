use serde::Deserialize;
use std::{error::Error, fs, path::PathBuf};

#[derive(Deserialize)] #[serde(default)] pub struct Config { pub general: General, pub logo: Logo, pub display: Display, pub modules: Modules }
#[derive(Deserialize)] #[serde(default)] pub struct General { pub color: bool, pub show_title: bool }
#[derive(Deserialize)] #[serde(default)] pub struct Logo { pub enabled: bool, #[serde(rename="type")] pub kind: String, pub position: String, pub path: String }
#[derive(Deserialize)] #[serde(default)] pub struct Display { pub separator: String, pub key_color: String, pub value_color: String }
#[derive(Deserialize)] #[serde(default)] pub struct Modules { pub os: bool, pub kernel: bool, pub hostname: bool, pub username: bool, pub shell: bool, pub wm: bool, pub cpu: bool, pub memory: bool, pub uptime: bool }
impl Default for Config { fn default()->Self { Self{general:General::default(),logo:Logo::default(),display:Display::default(),modules:Modules::default()} } }
impl Default for General { fn default()->Self {Self{color:true,show_title:true}} }
impl Default for Logo { fn default()->Self {Self{enabled:true,kind:"auto".into(),position:"left".into(),path:String::new()}} }
impl Default for Display { fn default()->Self {Self{separator:": ".into(),key_color:"green".into(),value_color:"white".into()}} }
impl Default for Modules { fn default()->Self {Self{os:true,kernel:true,hostname:true,username:true,shell:true,wm:true,cpu:true,memory:true,uptime:true}} }
pub fn path()->Result<PathBuf,Box<dyn Error>> { let home=std::env::var_os("HOME").ok_or("HOME is not set")?; Ok(PathBuf::from(home).join(".config/rfetch/config.toml")) }
pub fn load()->Result<Config,Box<dyn Error>> { let p=path()?; match fs::read_to_string(p) { Ok(s)=>toml::from_str(&s).map_err(|e| format!("invalid config: {e}").into()), Err(e) if e.kind()==std::io::ErrorKind::NotFound=>Ok(Config::default()), Err(e)=>Err(e.into()) } }
pub fn write_default()->Result<(),Box<dyn Error>> { let p=path()?; fs::create_dir_all(p.parent().ok_or("invalid config path")?)?; fs::write(p, "[general]\ncolor = true\nshow_title = true\n[logo]\nenabled = true\ntype = \"auto\"\nposition = \"left\"\n[display]\nseparator = \": \"\nkey_color = \"green\"\nvalue_color = \"white\"\n[modules]\nos = true\nkernel = true\nhostname = true\nusername = true\nshell = true\nwm = true\ncpu = true\nmemory = true\nuptime = true\n")?; Ok(()) }
