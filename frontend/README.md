# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

## Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

## Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```


## Troubleshooting 

- `dx build` error: `wasm-bindgen` use different bindgen format than this binary

```txt
it looks like the Rust project used to create this Wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust Wasm file schema version: 0.2.99
     this binary schema version: 0.2.97

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or
the wasm-bindgen dependency in the Rust project.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen --precise 0.2.97

don't forget to recompile your Wasm file! Alternatively, you can update the
binary with:

    cargo install -f wasm-bindgen-cli --version 0.2.99
```

Solution: 

```sh
cargo install -f wasm-bindgen-cli --version 0.2.99 
dx build
```