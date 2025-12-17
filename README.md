## Building the Berkeley Boot Loader

```
> cd riscv-pk
> mkdir build
> cd build
> ../configure --host=riscv64-unknown-elf --with-arch=rv64ima_zicsr_zifencei --enable-logo
> make
```

## Running the Emulator

The emulator prints to stdout as logging and stderr as output. Currently there is no way to stop the emulator so it's likely that it will show "Power off" and hang indefinitly, if this is the case just do ctrl+c to stop it

```
cargo run > log
```