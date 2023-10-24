## 🚴 Usage

### Run at `localhost:8080`
Install `wasm-pack`:
```
cargo install wasm-pack
```
Install npm packages:
```
cd www
npm install
```

Then in root dir:
```
wasm-pack build
cd www
npm run start
```

If you get following error:
```
error:03000086:digital envelope routines::initialization error
```
Then try running this and run `npm run start` again:
```
export NODE_OPTIONS=--openssl-legacy-provider
```


Refer to [Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html) if lost.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
