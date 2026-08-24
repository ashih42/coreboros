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

    pub fn dwarf_2() -> Self {
        let redcode = indoc! {r"
            ;name      dwarf_2
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

    pub fn nop() -> Self {
        let redcode = indoc::indoc! {"
            ;name      nop
            ;strategy  I do nothing.

            nop 0
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop` example redcode should be valid")
    }

    pub fn nop_20() -> Self {
        let redcode = indoc::indoc! {"
            ;name      nop_20
            ;strategy  I do nothing 20 times.

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

    pub fn looping_paper() -> Self {
        let redcode = indoc::indoc! {"
            ;name      looping_paper

            paper   mov    #5,       #0
            copy    mov    <paper,   {dest
                    jmn    copy,     paper
                    spl    >paper,   {-1277
            dest    jmz    5620,     *0
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop` example redcode should be valid")
    }

    pub fn blur_scanner() -> Self {
        let redcode = indoc::indoc! {"
            ;name      blur_scanner

            wptr    mov.b   scan,       #0
            scan    add     #4884,      #4884
            gate    mov     *bomb,      >wptr
                    jmz.f   scan,       @scan
                    jmn     wptr,       *wptr

            bomb    spl     0,          0
            clear   mov     dbmb,       >gate
                    djn.f   clear,      >gate
            dbmb    dat     <2667,      2-gate
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop` example redcode should be valid")
    }

    pub fn transposition_stone() -> Self {
        let redcode = indoc::indoc! {"
            ;name      transposition_stone

            inc     spl    #-1185,   <1185
            stone   mov    >1185,    1-1185
                    sub    inc,      stone
                    djn.f  stone,    <5555
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop` example redcode should be valid")
    }

    pub fn self_bombing_stone() -> Self {
        let redcode = indoc::indoc! {"
            ;name      self_bombing_stone

                    spl     #0,     0
            stone   mov     bomb,   hit+953*3382
            hit     add     #-953,  stone               ; bomb dropped here
                    djn.f   stone,  <5555

            bomb    dat     >-1,    {1
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop` example redcode should be valid")
    }

    pub fn self_vamping_vampire() -> Self {
        let redcode = indoc::indoc! {"
            ;name      self_vamping_vampire

            inc         spl    #2895,       <-2895
            vampire     mov    fang,        @fang         ; fang dropped here
                        sub    inc,         fang
                        djn.f  vampire,     *fang

                        dat    0,           0
                        dat    0,           0
                        dat    0,           0
                        dat    0,           0
                        dat    0,           0

            trap        mov    bomb+1,      <vampire-9
                        spl    trap
                        jmp    trap+1
            bomb        dat    <5334,       <2667

                        dat    0,           0
                        dat    0,           0
                        dat    0,           0

            fang        jmp    trap-vampire-2895,   <vampire+2895
        "};

        #[allow(clippy::expect_used, reason = "Redcode is valid 👌")]
        Self::from_text(redcode).expect("`nop` example redcode should be valid")
    }
}
