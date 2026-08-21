use strum_macros::{Display, EnumString};

// Reference: <https://corewars.org/docs/guide.html>
#[derive(Debug, Display, Clone, Copy, EnumString, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    DAT, // data (kills the process)

    MOV, // move (copies data from one address to another)

    ADD, // add (adds one number to another)
    SUB, // subtract (subtracts one number from another)
    MUL, // multiply (multiplies one number with another)
    DIV, // divide (divides one number with another)
    MOD, // modulus (divides one number with another and gives the remainder)

    JMP, // jump (continues execution from another address)
    JMZ, // jump if zero (tests a number and jumps to an address if it's 0)
    JMN, // jump if not zero (tests a number and jumps if it isn't 0)
    DJN, // decrement and jump if not zero (decrements a number by one, and jumps unless the result is 0)

    SPL, // split (starts a second process at another address)

    CMP, // compare (same as SEQ)
    SEQ, // skip if equal (compares two instructions, and skips the next instruction if they are equal)
    SNE, // skip if not equal (compares two instructions, and skips the next instruction if they aren't equal)
    SLT, // skip if lower than (compares two values, and skips the next instruction if the first is lower than the second)

    LDP, // load from p-space (loads a number from private storage space)
    STP, // save to p-space (saves a number to private storage space)

    NOP, // no operation (does nothing)
}

impl Opcode {
    /// Check if this `Opcode` is an operation that writes to the core.
    pub fn is_write(&self) -> bool {
        matches!(
            self,
            Self::MOV | Self::ADD | Self::SUB | Self::MUL | Self::DIV | Self::MOD
        )
    }
}

// TODO: Update tests

#[cfg(test)]
mod tests {
    use super::*;
    use Opcode::*;
    use std::str::FromStr as _;

    #[test]
    fn test_str_to_opcode() {
        assert_eq!(Opcode::from_str("MOV"), Ok(MOV));
        assert_eq!(Opcode::from_str("mov"), Ok(MOV));
        assert_eq!(Opcode::from_str("MoV"), Ok(MOV));
        assert_eq!(Opcode::from_str("mOV"), Ok(MOV));

        assert!(Opcode::from_str("mov ").is_err());
        assert!(Opcode::from_str(" mov").is_err());
        assert!(Opcode::from_str(" mov ").is_err());

        assert!(Opcode::from_str("move").is_err());
        assert!(Opcode::from_str("abc").is_err());
    }

    #[test]
    fn test_opcode_to_string() {
        assert_eq!("DAT", DAT.to_string());
        assert_eq!("MOV", MOV.to_string());
        assert_eq!("ADD", ADD.to_string());
        assert_eq!("SUB", SUB.to_string());
        assert_eq!("MUL", MUL.to_string());
        assert_eq!("DIV", DIV.to_string());
        assert_eq!("MOD", MOD.to_string());
        assert_eq!("JMP", JMP.to_string());
        assert_eq!("JMZ", JMZ.to_string());
        assert_eq!("JMN", JMN.to_string());
        assert_eq!("DJN", DJN.to_string());
        assert_eq!("SPL", SPL.to_string());
        assert_eq!("CMP", CMP.to_string());
        assert_eq!("SEQ", SEQ.to_string());
        assert_eq!("SNE", SNE.to_string());
        assert_eq!("SLT", SLT.to_string());
        assert_eq!("LDP", LDP.to_string());
        assert_eq!("STP", STP.to_string());
        assert_eq!("NOP", NOP.to_string());
    }
}
