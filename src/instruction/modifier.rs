use strum::AsRefStr;
use strum_macros::{Display, EnumString};

use crate::rng;

#[derive(Debug, Display, Clone, Copy, EnumString, Eq, PartialEq, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

impl Modifier {
    pub fn random_modifier() -> Self {
        static ALL_MODIFIERS: &[Modifier] = &[
            Modifier::A,
            Modifier::B,
            Modifier::AB,
            Modifier::BA,
            Modifier::F,
            Modifier::X,
            Modifier::I,
        ];

        let index = rng::rand_range(0, ALL_MODIFIERS.len());

        #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
        ALL_MODIFIERS[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Modifier::*;
    use std::str::FromStr as _;

    #[test]
    fn test_str_to_modifier() {
        assert_eq!(Modifier::from_str("AB"), Ok(AB));
        assert_eq!(Modifier::from_str("ab"), Ok(AB));
        assert_eq!(Modifier::from_str("Ab"), Ok(AB));
        assert_eq!(Modifier::from_str("aB"), Ok(AB));

        assert!(Modifier::from_str("AB ").is_err());
        assert!(Modifier::from_str(" AB").is_err());
        assert!(Modifier::from_str(" AB ").is_err());

        assert!(Modifier::from_str("ABB").is_err());
        assert!(Modifier::from_str("A B").is_err());
    }

    #[test]
    fn test_modifier_to_string() {
        assert_eq!("AB", AB.to_string());
        assert_eq!("BA", BA.to_string());
        assert_eq!("A", A.to_string());
        assert_eq!("B", B.to_string());
        assert_eq!("F", F.to_string());
        assert_eq!("X", X.to_string());
        assert_eq!("I", I.to_string());
    }
}
