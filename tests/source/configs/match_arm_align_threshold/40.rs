//@ rustfmt-match_arm_align_threshold: 40

fn basic(x: Tok) -> u32 {
    match x {
        Tok::Ident(s) => 1,
        Tok::Num(n) if n > 0 => 2,
        Tok::LParen => 3,
        Tok::VeryLongVariantName => 4,
    }
}

fn blank_line_splits_the_run(x: Tok) -> u32 {
    match x {
        Tok::A => 1,
        Tok::Bbbbbbbb => 2,

        Tok::C => 3,
        Tok::Dddddddd => 4,
    }
}

fn block_body_splits_the_run(x: Tok) -> u32 {
    match x {
        Tok::A => 1,
        Tok::Bbbbbbbb => 2,
        Tok::Blk => {
            let y = 1;
            y + 2
        }
        Tok::C => 3,
        Tok::Dddddddd => 4,
    }
}

fn threshold_exceeded(x: Tok) -> u32 {
    match x {
        Tok::A => 1,
        Tok::ThisVariantIsFarTooWideToAlignAgainstAShortOne => 2,
    }
}

fn attributes_opt_out(x: Tok) -> u32 {
    match x {
        Tok::A => 1,
        #[cfg(feature = "x")]
        Tok::Bbbbbbbb => 2,
        Tok::C => 3,
    }
}

fn or_patterns_and_bindings(x: Tok) -> u32 {
    match x {
        Tok::A | Tok::B => 1,
        Tok::N(n @ 1..=9) => 2,
        Tok::S([first, .., last]) => 3,
        Tok::Z => 4,
    }
}

fn wide_chars(x: Tok) -> u32 {
    match x {
        Tok::日本語 => 1,
        Tok::Ascii => 2,
        Tok::Z => 3,
    }
}

fn single_arm(x: Tok) -> u32 {
    match x {
        Tok::Only => 1,
    }
}

fn nested(x: Tok, y: Tok) -> u32 {
    match x {
        Tok::A => match y {
            Tok::Inner => 1,
            Tok::InnerLonger => 2,
        },
        Tok::Bbbbbbbb => 3,
    }
}

fn comment_then_blank_line_is_not_a_run_break(x: u8) -> u32 {
    match x {
        1 => a,
        // the blank line after this comment does not survive formatting
        22222 => b,
    }
}

fn blank_line_before_a_comment_is_a_run_break(x: u8) -> u32 {
    match x {
        1 => a,

        // c
        22222 => b,
    }
}

fn a_trailing_comment_may_extend_past_max_width(x: u8) -> u32 {
    match x {
        1 => aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, /* cccccccccccccc */
        222222222222222222 => b,
        3 => c,
    }
}

fn blank_line_within_the_comments_of_a_gap_is_not_a_run_break(x: u8) -> u32 {
    match x {
        1 => a,
        // a

        // b
        22222 => b,
    }
}

fn a_block_comment_then_a_blank_line_then_a_line_comment(x: u8) -> u32 {
    match x {
        1 => a,
        /* c */

        // d
        22222 => b,
    }
}

fn blank_line_inside_a_block_comment_is_not_a_run_break(x: u8) -> u32 {
    match x {
        1 => a,
        /* a

           b */
        22222 => b,
    }
}
