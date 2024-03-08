# axum-dioxus-shopping-list
example project for axum 0.7 + dioxus 0.4 SPA

## getting started
you need the dioxus-cli crate installed for its cargo wrapper `dx`,
as well as cargo-make and the `wasm32-unknown-unknown` target.

```shell
cargo install dioxus-cli
```

```shell
cargo install cargo-make
```

```shell
rustup target add wasm32-unknown-unknown
```

For styles, we use the tailwind and DaisyUI 'binary' so you need npm installed on your system and install the DaisyUI dep. 

```shell
npm install
```

After that, if you are on mac or linux, you should be able to start run the stack in dev mode with

```shell
./run.sh
```
which is only a tiny wrapper around cargo-make

This launches three processes in parallel: 
* a dev mode with hot reloading for the frontend on port *8080*
* tailwind file watch and compilation
* the backend on port *3000* with restart on file change (cargo watch)

Ctrl-C kills all three of these processes