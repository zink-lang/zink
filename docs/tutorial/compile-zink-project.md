# Compiling Zink Project

We have got a zink project after [creating-zink-project][create], now it's time to compile
it to EVM bytecode!

```bash
# Enter our project
cd my-awesome-project

# Build the project
elko build

# Check the outputs
ls target/zink
my-awesome-project.wasm my-awesome-project.bin
```

you'll see a `my-awesome-project.bin` file under `target/zink`, and that's it!

## How it works?

first, `elko` compiles the cargo project to WASM with:

```bash
cargo b --target wasm32-unknown-unknown --release
```

then, there will be some logic inside `elko`, running `wasm-opt` for our output
WASM binary

```bash
# if you have wasm-opt installed on your machine, you can try the same
mkdir -p target/zink
wasm-opt -O4 target/wasm32-unknown/unknown/release/my-awesome-project.wasm -o target/zink/my-awesome-project.wasm
```

finally we use `zinkc` to compile the wasm to EVM bytecode:

```bash
# For reproducing it in your command line
zinkc target/zink/my-awesome-project.wasm
mv my-awesome-project.bin target/zink
```

## Future plans (TODO)

1. Generate the ABI as well.
2. Add command for deploying the bytecode to EVM chain with RPC endpoints.
3. Test suite
4. ...

[create]: /tutorial/create-zink-project.html
