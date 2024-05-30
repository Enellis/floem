#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod helix;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ModalFlavour {
    #[default]
    None,
    Vim,
    Helix,
}

impl ModalFlavour {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModalFlavour::None => "none",
            ModalFlavour::Vim => "vim",
            ModalFlavour::Helix => "helix",
        }
    }

    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(ModalFlavour::None),
            "vim" => Some(ModalFlavour::Vim),
            "helix" => Some(ModalFlavour::Helix),
            _ => None,
        }
    }
}

impl std::fmt::Display for ModalFlavour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}
