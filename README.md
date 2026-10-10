# rustfmt aligned

This is a slop fork of [rustfmt](https://github.com/rust-lang/rustfmt).

The only difference is that four new features have been added:
* vertical alignment for match blocks
* vertical alignment for consecutive let statements
* option for vertically aligning the colons on field structs
* a knob for how much whitespace you're willing to put up with to get a table

## why?

this was made in response to [issue #894](https://github.com/rust-lang/rustfmt/issues/894) going unresolved for 10(ten!) years (one entire decade!) (it was opened in 2016!) (it's now 2026!!) (it's still open!!!), and [PR #3996](https://github.com/rust-lang/rustfmt/pull/3996) being closed without merge, citing "the current implementation needing a rewrite" in spite of:

1. Vertical alignment logic [already existing since 2017](https://github.com/rust-lang/rustfmt/commit/34b4a9d3c4ccd47a6dbc676dd0825693a85b9b01)
2. The "required" rewrite literally never happening
3. The working implementation for match arm alignment was `+152`/`-18` and touched *exactly 3(three) files* (basically only one file really, two of those files had only 4 lines added between them).
4. The [last maintainer response on #894](https://github.com/rust-lang/rustfmt/issues/894#issuecomment-452115725) (***January of the year 2019***) setting the bar as "if it can be done without too much change, then it would be good to have" (again, the diff was literally `~3`/`+152`/`-18`)
5. 14 comments from 13 different people in the *seven years* since, with *zero maintainer replies* (until I posted this criticism in the issue, anyway). The most recent comment asking for updates on the issue was January of this year.
6. out of 4,148 filed issues, this issue is the *8th most-reacted open issue* in the entire repo:

| reacts | id | name |
| ------ | -- | ---- |
| +221 | [#4991](https://github.com/rust-lang/rustfmt/issues/4991)  | [unstable option] imports_granularity |
| +186 | [#3863](https://github.com/rust-lang/rustfmt/issues/3863)  | Gives up on chains if any line is too long |
| +168 | [#5083](https://github.com/rust-lang/rustfmt/issues/5083)  | [unstable option] group_imports |
| +152 | [#8](https://github.com/rust-lang/rustfmt/issues/8)     | format macros |
| +119  | [#4091](https://github.com/rust-lang/rustfmt/issues/4091)  | Could rustfmt format Cargo.toml? |
| +95  | [#3347](https://github.com/rust-lang/rustfmt/issues/3347)  | [unstable option] wrap_comments |
| +89  | [#4306](https://github.com/rust-lang/rustfmt/issues/4306)  | optional programmer discretion on chain length |
| +89  | [#894](https://github.com/rust-lang/rustfmt/issues/894)   | ***Option to preserve match alignment*** |

again, this is out of **4,148** issues (top 0.2%).

## why do you care so much?

idk

## anyways

these additions were made with claude code in like, roughly 4 hours. I would write it myself and open a PR, but given the way the maintainers have handled this issue, it doesn't really seem worth the effort to try to contribute.

Use this if you feel like it, I'm not responsible for whatever happens to your codebase. I'm going to use it on my codebases though, so I'll be maintaining this fork and fixing any issues with it, though it seems to be in a stable state (It currently round trips cleanly on a [124k line rust codebase](https://github.com/km-clay/shed)). Be sure your working tree is clean before you run this.

Anyway, here are the features:

### Aligned `match` Blocks

`match` blocks can now be vertically aligned on the fat arrow:

```rust
match tok {
  Tok::Ident(s)            => 1,
  Tok::Num(n) if n > 0     => 2,
  Tok::LParen              => 3,
  Tok::VeryLongVariantName => 4,
}
```

This is configured in rustfmt.toml with `match_arm_align_threshold = <n>`, the default is 0, which means off

### Aligned consecutive `let` statements

Consecutive `let` statements can now be vertically aligned. This gives several consecutive bindings a table-like reading:

```rust
let     reader: Arc<dyn Sink> = procio::stdin_sink()?;
let mut writer: SinkIo        = SinkIo(procio::stdout_sink()?);

let     spec  : FloatSpec     = FloatSpec::from_args(&args).promote_err(args.cmd_span())?;
let     width : usize         = spec.width;
let     want  : usize         = limit.wanted();

let mut buf   : Vec<u8>       = vec![0u8; want * width];
let     got   : usize         = super::fill(&reader, &mut buf).promote_err(args.cmd_span())?;
let     whole : usize         = got / width;
let     rem   : usize         = got % width;
```

Each component of the let statement gets its own column. Very easy to tell at a glance what's mutable and what's not.

Configured using `let_align_threshold = <n>`, default is 0 (off).

### Struct field type alignment

This one is already a part of upstream, I'm just weird and prefer the padding to come before the colons, so the padding can come before the colons now:

```rust
struct FloatSpec {
  exp_bits     : usize,
  mant_bits    : usize,
  little_endian: bool,
}
```

Makes it more tabular imo. Configured using `struct_field_align_threshold = <n>` with `align_before_colon = true`.

### Excluding really long lines

A single long line can do stuff like this:

```rust
let lorem_ipsum_dolor_sit_amet = 1;
let consectetur                = 2;
let adipiscing                 = 3;
let elit_sed                   = 4;
```

Which is correct, but could reasonably be considered way too much padding for the short names.

This behavior can now be configured with `align_stretch_threshold` (default 0).

With `align_stretch_threshold = 10`, the long one is left out and the rest still get their table:

```rust
let lorem_ipsum_dolor_sit_amet = 1;
let consectetur = 2;
let adipiscing  = 3;
let elit_sed    = 4;
```

(this is with `let_align_threshold = 20`)

This is also why a run that can't align as a whole gets split into smaller tables instead of being
given up on.

Here are the numbers I settled on for [shed](https://github.com/km-clay/shed)'s formatting:

```toml
struct_field_align_threshold = 8
enum_discrim_align_threshold = 8
align_before_colon           = true

match_arm_align_threshold = 14
let_align_threshold       = 18
align_stretch_threshold   = 20
```
