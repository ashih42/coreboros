use std::str::FromStr;

use crate::rng;

// Note: Most enums in this codebase use `strum` to derive to_string() and from_str(),
// but `strum` cannot be used here to here handle string "}".
// This is an open issue: <https://github.com/Peternator7/strum/issues/363>
// For now, I use `derive_more` for to_string() and manually implement from_str().

/// Note: According to ICWS'94 Standard, if a mode specifier is not provided, it is Direct mode ('$').
/// Reference: <https://corewar.co.uk/standards/icws94.htm#2.4>
///
/// ICWS'94 Standard document is incomplete. Use the resource below instead.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/addressing_modes/>
#[derive(Debug, derive_more::Display, Default, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    #[display("#")]
    Immediate,

    #[default] // A missing mode symbol is treated as Direct mode.
    #[display("$")]
    Direct,

    #[display("*")]
    AIndirect,

    #[display("@")]
    BIndirect,

    #[display("{{")] // This means "{"
    AIndirectPreDecrement,

    #[display("}}")] // This means "}"
    AIndirectPostIncrement,

    #[display("<")]
    BIndirectPreDecrement,

    #[display(">")]
    BIndirectPostIncrement,
}

impl AddressingMode {
    pub fn random_addressing_mode() -> Self {
        static ALL_ADDRESSING_MODES: &[AddressingMode] = &[
            AddressingMode::Immediate,
            AddressingMode::Direct,
            AddressingMode::AIndirect,
            AddressingMode::BIndirect,
            AddressingMode::AIndirectPreDecrement,
            AddressingMode::AIndirectPostIncrement,
            AddressingMode::BIndirectPreDecrement,
            AddressingMode::BIndirectPostIncrement,
        ];

        let index = rng::rand_range(0, ALL_ADDRESSING_MODES.len());

        ALL_ADDRESSING_MODES[index]
    }
}

impl FromStr for AddressingMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Immediate),
            "$" => Ok(Self::Direct),
            "*" => Ok(Self::AIndirect),
            "@" => Ok(Self::BIndirect),
            "{" => Ok(Self::AIndirectPreDecrement),
            "}" => Ok(Self::AIndirectPostIncrement),
            "<" => Ok(Self::BIndirectPreDecrement),
            ">" => Ok(Self::BIndirectPostIncrement),
            _ => Err("Invalid AddressingMode string"),
        }
    }
}

impl AsRef<str> for AddressingMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::Immediate => "#",
            Self::Direct => "$",
            Self::AIndirect => "*",
            Self::BIndirect => "@",
            Self::AIndirectPreDecrement => "{",
            Self::AIndirectPostIncrement => "}",
            Self::BIndirectPreDecrement => "<",
            Self::BIndirectPostIncrement => ">",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use AddressingMode::*;

    #[test]
    fn test_str_to_addressing_mode() {
        assert_eq!(AddressingMode::from_str("#"), Ok(Immediate));
        assert_eq!(AddressingMode::from_str("$"), Ok(Direct));
        assert_eq!(AddressingMode::from_str("*"), Ok(AIndirect));
        assert_eq!(AddressingMode::from_str("@"), Ok(BIndirect));
        assert_eq!(AddressingMode::from_str("{"), Ok(AIndirectPreDecrement));
        assert_eq!(AddressingMode::from_str("}"), Ok(AIndirectPostIncrement));
        assert_eq!(AddressingMode::from_str("<"), Ok(BIndirectPreDecrement));
        assert_eq!(AddressingMode::from_str(">"), Ok(BIndirectPostIncrement));

        assert!(AddressingMode::from_str(" #").is_err());
        assert!(AddressingMode::from_str("# ").is_err());
        assert!(AddressingMode::from_str(" # ").is_err());
        assert!(AddressingMode::from_str("##").is_err());
        assert!(AddressingMode::from_str("?").is_err());
    }

    #[test]
    fn test_addressing_mode_to_string() {
        assert_eq!("#", Immediate.to_string());
        assert_eq!("$", Direct.to_string());
        assert_eq!("*", AIndirect.to_string());
        assert_eq!("@", BIndirect.to_string());
        assert_eq!("{", AIndirectPreDecrement.to_string());
        assert_eq!("}", AIndirectPostIncrement.to_string());
        assert_eq!("<", BIndirectPreDecrement.to_string());
        assert_eq!(">", BIndirectPostIncrement.to_string());
    }
}
