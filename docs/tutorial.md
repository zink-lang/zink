# Tutorial

This tutorial walks through creating a simple add-two program and compiles it with description how everything works.

- [Creating `add-two`](/tutorial/create-zink-project)
- [Compiling `add-two`](/tutorial/compile-zink-project)

For the overall instructions, with [elko](/cli/elko.html) installed, all you need is:

```bash
# Install zink toolchain
cargo install zinkup

# Create project
elko new add-two

# Compile it
cd add-two && elko build
ls target/add-two.bin
```
