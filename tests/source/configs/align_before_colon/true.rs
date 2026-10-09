// rustfmt-align_before_colon: true
// rustfmt-struct_field_align_threshold: 40
// rustfmt-enum_discrim_align_threshold: 40
// rustfmt-let_align_threshold: 40

#[derive(Debug, Clone, Copy)]
struct FloatSpec {
    exp_bits: usize,
    mant_bits: usize,
    little_endian: bool,
}

struct Visibility {
    pub a: u32,
    pub(crate) bb: u32,
    ccc: u32,
}

struct BlankLineSplitsTheGroup {
    a: u32,
    bbbbbbbb: u32,

    c: u32,
    dddddddd: u32,
}

struct Tuple(u32, u64);

struct Unit;

enum Discriminants {
    A = 1,
    Bcdef = 22,
    C = 333,
}

fn struct_literal() {
    let x = Foo {
        exp_bits: 1,
        mant_bits: 2,
        little_endian: true,
    };
}

fn shorthand_fields_opt_the_literal_out() {
    let x = Foo {
        exp_bits: 1,
        mant_bits: 2,
        little_endian: true,
        shorthand,
    };
}

fn let_bindings() {
    let a: u8 = 1;
    let bbbb: Vec<String> = vec![];
    let mut c = 3;
}

struct WideChars {
    日本語: u32,
    ascii: u32,
    z: u32,
}

fn literal_pad_is_measured_in_columns_not_bytes() {
    let _ = S { aé: 1, bb: 2, c字字: 3 };
}

struct AccentedDefinition {
    café: u8,
    xx: u16,
    naïve: u32,
}
