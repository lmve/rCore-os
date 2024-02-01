### rCore 从0到1开发日记
在此记录rCore的开发过程
while the world sleeps, you dream!
### Day one
在裸机上实现 Hello world
系统总体架构如下图：
！[](https://rcore-os.cn/rCore-Tutorial-Book-v3/_images/lib-os-detail.png)
大致过程为 Qemu 把包含 app 与 libos 的 image 加载到内存中,RustSBI(bootloader)完成基本硬件初始化并跳转到 libos 起始地址， libos 进行运行 app 前的初始化工作(建立栈空间、清零bss段)，之后跳转到 app 去执行。app 在执行过程中会通过函数调用的方式得到 libos 提供的 os 服务，如输出字符串等，避免 app 与硬件直接交互的繁琐过程。
**应用程序执行环境**
操作系统上层的应用程序运行，需要下面多层次的执行环境栈的支持，如下图下层的执行环境支持上层代码的运行，白块表示各级的执行环境，黑块表示相邻两层间的接口。
！[](https://rcore-os.cn/rCore-Tutorial-Book-v3/_images/app-software-stack.png)
**QEMU 模拟器**
使用软件qemu-system-riscv64来模拟一台64位RISC-V架构的计算机。
> 启动qemu 
``` 
qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-nukonwn-none-elf/release/os.bin,addr=0x80200000
```
- -machine virt 表示将模拟的64位RISC-V计算机设置为名为virt的虚拟计算机。
- -nographic 表示模拟器不需提供图形化界面。
- -bios 可以设置QEMU开机时的引导程序(bootloader)。
- -device 中的 loader 属性可将一个宿主机上的文件在开机前载入到QEMU的物理内存指定位置，file、addr属性可以设置待载入文件的路径以及文件载入到物理内存的地址。
**程序的内存布局**
！[](https://rcore-os.cn/rCore-Tutorial-Book-v3/_images/MemoryLayout.png)

- 已初始化数据段保存程序中那些已初始化的全局数据，分为 .rodata 和 .data 两部分。前者存放只读的全局数据，通常是一些常数或者是 常量字符串等；而后者存放可修改的全局数据。
- 未初始化数据段 .bss 保存程序中那些未初始化的全局数据，通常由程序的加载者代为进行零初始化，即将这块区域逐字节清零；
- 堆 （heap）区域用来存放程序运行时动态分配的数据，如 C/C++ 中的 malloc/new 分配到的数据本体就放在堆区域，它向高地址增长；
- 栈 （stack）区域不仅用作函数调用上下文的保存与恢复，每个函数作用域内的局部变量也被编译器放在它的栈帧内，它向低地址增长。
**编译流程**
> 
1. 编译器 (Compiler) 将每个源文件从某门高级编程语言转化为汇编语言，注意此时源文件仍然是一个 ASCII 或其他编码的文本文件
2. 汇编器 (Assembler) 将上一步的每个源文件中的文本格式的指令转化为机器码，得到一个二进制的 目标文件 (Object File)；
3. 链接器 (Linker) 将上一步得到的所有目标文件以及一些可能的外部目标文件链接在一起形成一个完整的可执行文件。
汇编器输出的每个目标文件都有一个独立的程序内存布局，它描述了目标文件内各段所在的位置。而链接器所做的事情是将所有输入的目标文件整合成一个整体的内存布局。在此期间链接器主要完成两件事情。
*第一件事情是将来自不同目标文件的段在目标内存布局中重新排布*
！[来自不同目标文件的段的重新排布](https://rcore-os.cn/rCore-Tutorial-Book-v3/_images/link-sections.png)
*第二件事情是将符号替换为具体地址*

**编写内核第一条指令**
**让内核支持函数调用**
**基于SBI服务完成输出与关机**





