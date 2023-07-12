Excerpt from the book
```
$  cargo new hello_cargo --bin
We passed the --bin argument to cargo new because our goal is to make an executable application, as opposed to a library. Executables are binary executable files often called just binaries. We’ve given hello_cargo as the name for our project, and Cargo creates its files in a directory of the same name that we can then go into.
```

Creating a rust project through cargo creates an src directory where our code lives and a Cargo.toml file at top level/root.

To build projects through cargo (as opposed to rustc)
```
cargo build
```
This will build the code into `./target/debug/{project_name}` (`./target/debug/hello_cargo`) in this case. Building projects with cargo also creates Cargo.lock file which keeps track of dependencies (and their versions). We usually would never have to edit this file on our own. The compiled code can be executed either by running `./target/debug/{project_name}` or by simply doing
```
cargo run
```
When the project is release ready, running
```
cargo build --release
```
will create an executable in `./target/release/` folder.