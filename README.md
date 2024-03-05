# axum-dioxus-shopping-list
example project for axum 0.7 + dioxus 0.4 SPA

## getting started
you need the dioxus-cli crate installed for its cargo wrapper `dx`,
as well as the `wasm32-unknown-unknown` target.

```shell
cargo install dioxus-cli
```

```shell
rustup target add wasm32-unknown-unknown
```

After that, if you are on mac or linux, you should be able to start run the stack with

```shell
./run.sh
```
This launches two processes in parallel: a dev mode with hot reloading for the frontend on port *8080*,
and the backend on port *3000*
- [ ] TODO: Tailwind compilation / watch
- [ ] TODO: the run script is currently flawed, the 'backend' process has to be killed separately after ctrl-c