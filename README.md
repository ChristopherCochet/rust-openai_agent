# rust-openai-agent project
Coursera GPT project in Rust

# Example of Makefile usage
```
> make rust-version

Rust command-line utility versions:
rustc --version                         #rust compiler
rustc 1.70.0 (90c541806 2023-05-31)
cargo --version                         #rust package manager
cargo 1.70.0 (ec8a8a0ca 2023-04-25)
rustfmt --version                       #rust code formatter
rustfmt 1.5.2-stable (90c5418 2023-05-31)
rustup --version                        #rust toolchain manager
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.70.0 (90c541806 2023-05-31)`
clippy-driver --version         #rust linter
clippy 0.1.70 (90c5418 2023-05-31)
```


# Example of CLI Marco-Polo usage

## Using Cargo
```
cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello-marco --help`
A Marco Polo game.

Usage: hello-marco [COMMAND]

Commands:
  marco  A Marco Polo game.
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
$ cargo run --  marco --name "Marco"
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello-marco marco --name Marco`
Polo
```

```
 cargo run --  marco --name "aaa"
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/hello-marco marco --name aaa`
Marco
```

## Using The Rust Executable
```
$ target/debug/hello-marco --help
A Marco Polo game.

Usage: hello-marco [COMMAND]

Commands:
  marco  A Marco Polo game.
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
target/debug/calc 

https://sturdy-space-system-rr57xvx59gjfp945-8080.app.github.dev/add/3/3
return 6
```
## References

* [auto-gipiti](https://www.udemy.com/course/autogpt-gpt4-code-writing-ai/)
* Noah Gift [hello rust](https://github.com/nogibjj/hello-rust)
* Noah Gift [rust new project template](https://github.com/nogibjj/rust-new-project-template)
* Noah Gift [coursera] (https://www.coursera.org/lecture/devops-dataops-mlops-duke/github-copilot-enabled-rust-programming-klFcu)
* Rust language online reference (https://doc.rust-lang.org/cargo/index.html)
