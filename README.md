Using https://github.com/riscv-software-src/riscv-pk:



```
> mkdir build
> cd build
> ../configure --host=riscv64-unknown-elf --with-arch=rv64im_zicsr_zifencei --enable-logo
> make; cp bbl.bin ~/path/to/riscv-emulator/
```