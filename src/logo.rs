use std::fs;

pub fn get(kind: &str, custom: &str, id: &str, like: &str) -> String {
    if !custom.is_empty() {
        if let Ok(logo) = fs::read_to_string(custom) {
            return logo;
        }
    }

    let names: Vec<&str> = if kind != "auto" {
        vec![kind]
    } else {
        std::iter::once(id).chain(like.split_whitespace()).collect()
    };

    for name in names {
        let logo = match name {
            "arch" => include_str!("../logos/arch.txt"),
            "debian" => include_str!("../logos/debian.txt"),
            "ubuntu" => include_str!("../logos/ubuntu.txt"),
            "fedora" => include_str!("../logos/fedora.txt"),
            "gentoo" => include_str!("../logos/gentoo.txt"),
            "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => {
                include_str!("../logos/opensuse.txt")
            }
            "void" => include_str!("../logos/void.txt"),
            "alpine" => include_str!("../logos/alpine.txt"),
            _ => continue,
        };
        return logo.to_string();
    }

    include_str!("../logos/default.txt").to_string()
}
