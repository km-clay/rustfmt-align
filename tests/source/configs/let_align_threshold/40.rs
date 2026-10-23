//@ rustfmt-let_align_threshold: 40
//@ rustfmt-align_stretch_threshold: 10

fn basic() {
    let spec = FloatSpec::from_args(&args)?;
    let width = spec.width;

    let reader = procio::stdin_sink()?;
    let mut writer = SinkIo(procio::stdout_sink()?);

    let want = limit.wanted();
    let mut buf = vec![0u8; want * width];
    let got = fill(&reader, &mut buf)?;
    let whole = got / width;
    let rem = got % width;
}

fn type_annotations() {
    let a: u8 = 1;
    let bbbb: Vec<String> = vec![];
    let mut c = 3;
}

fn uninit_breaks_the_run() {
    let aaaaaaaa = 1;
    let b;
    let cccccccc = 3;
    let d = 4;
}

fn let_else_breaks_the_run() {
    let aaaaaaaa = 1;
    let Some(b) = opt else { return };
    let cccccccc = 3;
    let d = 4;
}

fn other_statements_break_the_run() {
    let aaaaaaaa = 1;
    let b = 2;
    println!("between");
    let cccccccc = 3;
    let d = 4;
}

fn nested_blocks_align_separately() {
    let outer = {
        let x = 1;
        let yy = 2;
        x + yy
    };
    let after = 3;
}

fn nothing_to_pad() {
    let a = 1;
    let b = 2;
}

fn wide_chars() {
    let 日本語 = 1;
    let ascii = 2;
    let z = 3;
}

fn ref_mut_is_not_the_mut_column() {
    let ref mut a = x;
    let bbbb = 2;
    let mut c = 3;
}

fn tuple_patterns() {
    let (a, b) = pair();
    let (mut c, d) = pair();
    let eeee = 5;
}

fn a_trailing_comment_may_extend_past_max_width() {
    let a = 1; // a comment long enough that padding carries it past the maximum width
    let bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb = 2;
}

fn a_standalone_comment_does_not_end_a_run() {
    let a = 1;
    let bbbb = 2;
    // a standalone comment after the final binding of the block
}

fn an_unannotated_binding_holds_the_type_column_open() {
    let a: u8 = 1;
    let bbbb = 2;
    let cc: usize = 3;
    let d = 4;
}

fn a_wide_type_spread_leaves_the_colons_unaligned() {
    let a: u8 = 1;
    let b: Arc<dyn Fn(usize) -> Result<Vec<String>, Error>> = f();
    let c: u8 = 3;
}

fn type_widths_are_display_columns() {
    let a: 日本語 = x;
    let bbbb: u8 = y;
}

fn a_lone_annotation_does_not_reserve_a_type_column() {
    let mut abbreviate_home = true;
    let mut one_per_line = false;
    let mut one_per_line_indexed = false;
    let mut clear_stack = false;
    let mut target_idx: Option<StackIdx> = None;
    let blame = args.span();
}

fn two_annotations_do_reserve_one() {
    let mut abbreviate_home = true;
    let mut clear_stack: bool = false;
    let mut target_idx: Option<StackIdx> = None;
    let blame = args.span();
}

fn a_wide_binding_may_not_stretch_the_table_for_the_rest() {
    let lorem_ipsum_dolor_sit_amet = 1;
    let sit = 2;
    let amet = 3;
}
