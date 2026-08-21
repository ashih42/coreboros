use indoc::indoc;

use crate::warrior::Warrior;

impl Warrior {
    pub fn dwarf() -> Self {
        let redcode = indoc::indoc! {"
            ;name      dwarf
            ;strategy  I bomb every 4th cell.

            ADD #4, 3
            MOV 2, @2
            JMP -2
            DAT #0, #0
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`dwarf` example redcode should be valid")
    }

    pub fn dwarf_verbose() -> Self {
        let redcode = indoc! {r"
            ;name      dwarf_verbose
            ;strategy  I also bomb every 4th cell.

            org        dwarf

            bomb       dat #0
            dwarf      add #4,   bomb
                       mov bomb, @bomb
                       jmp dwarf
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`dwarf_verbose` example redcode should be valid")
    }

    pub fn imp() -> Self {
        let redcode = indoc::indoc! {"
            ;name      imp
            ;strategy  I copy a single instruction forward.

            mov 0, 1
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`imp` example redcode should be valid")
    }

    pub fn imp_factory() -> Self {
        let redcode = indoc::indoc! {"
            ;name      imp_factory
            ;strategy  I make imps.

            factory    spl factory
            imp        mov imp, imp+1
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`imp_factory` example redcode should be valid")
    }

    // pub fn nope() -> Self {
    //     let redcode = indoc::indoc! {"
    //         ;name      nope
    //         ;strategy  I don't know.

    //         nop 0
    //     "};

    //     Self::from_text(redcode).unwrap()
    // }

    pub fn nop_20() -> Self {
        let redcode = indoc::indoc! {"
            ;name      nop_20
            ;strategy  I nop 20 times.

            nop 0
            nop 1
            nop 2
            nop 3
            nop 4
            nop 5
            nop 6
            nop 7
            nop 8
            nop 9
            
            nop 10
            nop 11
            nop 12
            nop 13
            nop 14
            nop 15
            nop 16
            nop 17
            nop 18
            nop 19
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop_20` example redcode should be valid")
    }
}
