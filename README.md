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

## Testing the web api template

### Test task creation (POST)
Run the web template
```
/workspaces/rust-openai_agent/web_template (main) $ cargo run
warning: `web_template` (bin "web_template") generated 4 warnings (run `cargo fix --bin "web_template"` to apply 3 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/web_template`
```

Test the API
```
/workspaces/rust-openai_agent (main) $ curl -i -X POST   -H "Content-type: application/json"   -H "Accept: application/json"   -d '{"id":1,"name": "my first task", "completed": fal
se}'   "http://localhost:8181/task"
HTTP/1.1 200 OK
content-length: 0
access-control-allow-credentials: true
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
date: Sat, 28 Oct 2023 01:45:34 GMT
```

### Test specific task retrieval (GET)
Run the web template
```
/workspaces/rust-openai_agent/web_template (main) $ cargo run
warning: `web_template` (bin "web_template") generated 4 warnings (run `cargo fix --bin "web_template"` to apply 3 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/web_template`
```

```
/workspaces/rust-openai_agent (main) $  curl -i -X GET   -H "Content-type: application/json"   -H "Accept: application/json"   "http://localhost:8181/task/1"
HTTP/1.1 200 OK
content-length: 49
access-control-expose-headers: content-type
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
access-control-allow-credentials: true
content-type: application/json
date: Sat, 28 Oct 2023 02:07:27 GMT

{"id":1,"name":"my first task","completed":false}
```

### Test all tasks retrieval (GET)
Run the web template
```
/workspaces/rust-openai_agent/web_template (main) $ cargo run
warning: `web_template` (bin "web_template") generated 4 warnings (run `cargo fix --bin "web_template"` to apply 3 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/web_template`
```

```
/workspaces/rust-openai_agent (main) $ curl -i -X GET   -H "Content-type: application/json"   -H "Accept: application/json"   "http://localhost:8181/task/"
HTTP/1.1 200 OK
content-length: 98
access-control-expose-headers: content-type
access-control-allow-credentials: true
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
content-type: application/json
date: Sat, 28 Oct 2023 02:16:12 GMT

[{"id":1,"name":"my first task","completed":false},{"id":2,"name":"my 2nd task","completed":true}]
```

### Update an Existing Task (PUT)

```
/workspaces/rust-openai_agent (main) $ curl -i -X PUT   -H "Content-type: application/json"   -H "Accept: application/json"   -d '{"id":2,"name": "updated my 2nd task", "completed":false}'   "http://localhost:8181/task"
HTTP/1.1 200 OK
content-length: 0
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
access-control-allow-credentials: true
date: Sat, 28 Oct 2023 02:33:49 GMT
```

### Delete an Existing Task (DELETE)

```
/workspaces/rust-openai_agent (main) $ curl -i -X DELETE   -H "Content-type: application/json"   -H "Accept: application/json"   "http://localhost:8181/task/2"
HTTP/1.1 200 OK
content-length: 0
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
access-control-allow-credentials: true
date: Sat, 28 Oct 2023 02:41:31 GMT
```

### Register an new user (POST)

```
curl -i -X POST   -H "Content-type: application/json"   -H "Accept: application/json"   -d '{"id":1, "name": "chris", "password": "123password"}'   "http://localhost:8181/register"
HTTP/1.1 200 OK
content-length: 0
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
access-control-allow-credentials: true
date: Sat, 28 Oct 2023 03:01:05 GMT
```

### Login an existing user (POST)

Successful login
```
/workspaces/rust-openai_agent (main) $ curl -i -X POST   -H "Content-type: application/json"   -H "Accept: application/json"   -d '{"id":1, "name": "chris", "password": "123password"}'   "http://localhost:8181/login"
HTTP/1.1 200 OK
content-length: 12
access-control-allow-credentials: true
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
date: Sat, 28 Oct 2023 03:02:49 GMT

Logged in !!
```


Wrong password 
```
 /workspaces/rust-openai_agent (main) $ curl -i -X POST   -H "Content-type: application/json"   -H "Accept: application/json"   -d '{"id":1, "name": "chris", "password": "31
23password"}'   "http://localhost:8181/login"
HTTP/1.1 400 Bad Request
content-length: 30
access-control-allow-credentials: true
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
date: Sat, 28 Oct 2023 03:06:15 GMT

Invalid username or password !
```

## References

* [auto-gipiti](https://www.udemy.com/course/autogpt-gpt4-code-writing-ai/)
* Noah Gift [hello rust](https://github.com/nogibjj/hello-rust)
* Noah Gift [rust new project template](https://github.com/nogibjj/rust-new-project-template)
* Noah Gift [coursera] (https://www.coursera.org/lecture/devops-dataops-mlops-duke/github-copilot-enabled-rust-programming-klFcu)
* Rust language online reference (https://doc.rust-lang.org/cargo/index.html)
