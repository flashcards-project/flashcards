# Contributing to Flashcards

Thank you for taking the time to contribute to the flashcards project!

You can:
- Report a bug
- Suggest a feature
- Fix something in .md files (e. g. a typo)
- Try fixing already found bug
- Try implementing suggested feature
- Help with answering some issues
- Test untested things and write tests
- Help with development organization (e. g. deal with Docker or github actions)

And many many other things!

Please, before making a contribution get familiar with sections and things
described in this file.

Also note that you must know [Rust][rust] at least a little bit, if you want to
contribute to the main code base. You don't have to know it, if you just want to
report a bug or suggest a feature (see [Reporting or Requesting
section][reporting] below) or do something not related to the main code base.

[rust]: https://www.rust-lang.org
[reporting]: #reporting-or-requesting

## Conduct

The Flashcards Project adheres to the [Rust Code of Conduct][coc]. This
describes the _minimum_ behavior expected from all contributors. Instances of
violations of the Code of Conduct can be reported by contacting the project
owner at [markmelix@gmail.com](mailto:markmelix@gmail.com).

[coc]: https://www.rust-lang.org/policies/code-of-conduct

## Reporting or Requesting

If you want to report a bug or request a feature, use [issues repository
page][issues] to do this. Mark a report with corresponding labels.

[issues]: https://github.com/flashcards-project/flashcards/issues

## Contributing to code

### Preparing workspace

You have to set up some things before coding.

1. [Install latest Rust toolchain manager (rustup)][install rust]. If you have
   rustup already installed, update toolchains to the latest version with
   command: `rustup update`.
2. Install [rustfmt][rustfmt] with this command: `rustup component add
   rustfmt`. Make sure that you can easily format your code using rustfmt in
   your editor and rustfmt formats code with settings defined in
   [rustfmt.toml](./rustfmt.toml) file.
3. Set your code editor to use `cargo clippy` instead of `cargo check` to check
   your code. If you have no [rust-clippy][rust-clippy] installed, install it
   with `rustup component add clippy` command.

[install rust]: https://www.rust-lang.org/tools/install
[rustfmt]: https://github.com/rust-lang/rustfmt
[rust-clippy]: https://github.com/rust-lang/rust-clippy

### Branching workflow

This project uses [git flow branching workflow][git flow]. Get familiar with it.
Thus, repository always has two branches: _develop_ and _main_. _main_ branch
has only tagged and completed releases. _develop_ one has the main code base.
There must also be _feature_, _release_ and _hotfix_ branches. I **really do
recommend** you to learn of this branching model from the link I mentioned
earlier and give it a try!

[git flow]: https://nvie.com/posts/a-successful-git-branching-model/#the-main-branches

### Commit message style

Please, when you write a commit message, make sure it adheres to [Conventional
Commits][conv commits] specification.

[conv commits]: https://www.conventionalcommits.org/en/v1.0.0/

### Code style

All code style rules are written in [rustfmt.toml file](./rustfmt.toml). You may
not reading it, just use `rustfmt` tool on every file after changing it or be
sure to run `cargo fmt` before every commit to format all project files to
satisfy the project code style.

You can even create git hook which will be doing that for you automatically
before every commit. Create file *.git/hooks/pre-commit* in the repository root
and fill it with the following content:

```sh
#!/usr/bin/sh
cargo fmt # format all files
git add . # add all changed files to the git index
```

And make this file executable (`chmod +x .git/hooks/pre-commit`).

### Documenting

Try to write documentation and docs examples for every module, struct, field,
enum and function. If you don't know how to write documentation in Rust, read
[rustdoc book][rustdoc] and get familiar with `cargo doc` command.

After you write documentation example, you should test it (see Testing section
below).

[rustdoc]: https://doc.rust-lang.org/rustdoc

### Testing

Before writing tests get familiar with [test organization in Rust][test-org] and
[cargo test][cargo-test] command. This project follows almost everything that's
described in these two documents.

Thus, if you want to test all project at once, use `cargo test` command.

[test-org]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
[cargo-test]: https://doc.rust-lang.org/cargo/commands/cargo-test.html

### Making a Changelog

This project uses ["Keep a Changelog system"][keep-a-changelog] to keep a
changelog.

[keep-a-changelog]: https://keepachangelog.com

Try writing to the [changelog](./CHANGELOG.md) every time you change project
version number.

### Releasing

Everything related to releasing is the same as described in the [git flow
manual][git flow]. Just create _release-x.y.z_ branch for preparation before a
new production release, fix minor bugs, if there're some and merge it to the
_main_ branch and make a tag with name vX.Y.Z (e. g. v1.2.3).

[git flow]: https://nvie.com/posts/a-successful-git-branching-model/#release-branches

### What's next?

If you got familiar with the things described above, start contributing!

If you'd like to start contributing to the code, but don't know what to do
first, see [good first issues][good first issues] and try taking one.

[good first issues]: https://github.com/flashcards-project/flashcards/contribute
