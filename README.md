### rCore os 从零到一

### day one
从最简单的Rust应用程序入手，深入理解其下的执行环境，分析编译器和操作系统为应用程序开发和运行提供的便利条件。
从Hello World程序开始逐步构建操作系统。
- **实现内核第一条指令**
```
cargo build --release
release 模式生成了内核的可执行文件，位于 os/target/riscv64gc.../release/os
使用如下命令可以丢弃内核可执行文件中的元数据得到内核镜像：
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
运行
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
```
### 努力更新中 os is loading...

### 学习资料
[rCore-Tutorial-Book 第三版](https://rcore-os.cn/rCore-Tutorial-Book-v3/)
### For every coder
While the world sleeps, you dream!
Be difference, leave something worth!
