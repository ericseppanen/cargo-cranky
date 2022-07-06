# cargo-cranky

I wish that I could check in a file that would specify various [`clippy`][clippy] lints for my entire Cargo workspace, and have that be applied to all the crates, libraries, binaries, and examples.

That's not possible with **clippy**, but it is possible with **cranky**!

cargo-cranky is just a wrapper around cargo-clippy; it examines your `Cranky.toml` config file, and constructs the necessary cargo-clippy command line.

For example, if `Cranky.toml` contains this:

```txt
warn = [
  "empty_structs_with_brackets",
  "cast_possible_truncation",
]
```

and I run `cargo cranky`, I get those extra lints:
```txt
warning: found empty brackets on struct declaration
  --> src/main.rs:11:12
   |
11 | struct Unit {}
   |            ^^^
```

```txt
warning: casting `u64` to `u8` may truncate the value
  --> src/main.rs:23:9
   |
23 |         x as u8
```

This is exactly the same as manually running `cargo clippy` with the extra parameters `--warn clippy::empty_structs_with_brackets` and `--warn clippy::cast_possible_truncation`.

You may find some useful clippy lints for your project in the [clippy documentation][clippy-docs]. I recommend browsing the "pedantic" and "restriction" groups.

### Installing

`cargo install cargo-cranky`

### FAQ

**Does it work with vscode?**

Yes! Just type `cranky` into the "Check On Save: Command" setting, or drop this into `settings.json`:
```txt
{
    "rust-analyzer.checkOnSave.command": "cranky"
}
```

[clippy]: https://github.com/rust-lang/rust-clippy#readme
[clippy-docs]: https://rust-lang.github.io/rust-clippy/stable/index.html
