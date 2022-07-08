# cargo-cranky

I wish that I could check in a file that would specify various [`clippy`][clippy] lints for my entire Cargo workspace, and have that be applied to all the crates, libraries, binaries, and examples.

That's not possible with **clippy**, but it is possible with **cranky**!

cargo-cranky is just a wrapper around cargo-clippy; it examines your `Cranky.toml` config file, and constructs the necessary cargo-clippy command line.

For example, if `Cranky.toml` contains this:

```toml
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

### Configuring

Create a file called `Cranky.toml` at the top of your project tree. The file can contain keys `allow`, `warn`, or `deny` that contain an array of clippy lint names.

Example:
```toml
deny = [
  "needless_return",
]

warn = [
  "empty_structs_with_brackets",
  "cast_possible_truncation",
]

allow = [
  "double_comparisons",
]
```

Note: in the case of overlap, `allow` will always override `warn`, which in turn will always override `deny`. The order of these fields in `Cranky.toml` has no effect.

### FAQ

**Does it work with vscode?**

Yes! Just type `cranky` into the "Check On Save: Command" setting, or drop this into `settings.json`:
```txt
{
    "rust-analyzer.checkOnSave.command": "cranky"
}
```

Set it back to "check" (or "clippy") to return to the previous behavior.

**Is this reckless or non-idiomatic?**

That depends on how you use it. If your goal is to enforce a non-idiomatic coding style, that's probably not a great idea.

Another questionable choice would be using `Cranky.toml` to suppress clippy lints that are enabled by default, though I'm sure there are rare conditions where that might make sense. It's still probably better to do that using the `#[allow(clippy::some_lint)]` syntax, since that gives you a chance to add a comment explaining your reasoning.

The main goal of this tool is to make it easier to enable additional clippy lints, that improve code maintainability or safety (i.e. `cast_possible_truncation`).

**I have ~~complaints~~ suggestions!**

Please [file a GitHub issue][github-issue] if you have ideas that could make this tool better.


[github-issue]: https://github.com/ericseppanen/cargo-cranky/issues
[clippy]: https://github.com/rust-lang/rust-clippy#readme
[clippy-docs]: https://rust-lang.github.io/rust-clippy/stable/index.html
