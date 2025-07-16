use std::{ffi::OsStr, fmt::Display};

pub enum Language {
    Rust,
    TOML,
    LinkerScript,
}

impl Language {
    pub fn from_ext(ext: &OsStr) -> Option<Self> {
        let ext = ext.to_str()?;

        let res = match ext {
            "rs" | "rs.in" => Self::Rust,
            "toml" => Self::TOML,
            "ld" | "lds" | "x" => Self::LinkerScript,
            _ => return None,
        };

        Some(res)
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "Rust"),
            Self::TOML => write!(f, "TOML"),
            Self::LinkerScript => write!(f, "Linker Script"),
        }
    }
}
