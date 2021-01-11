# Kernel

## Plan/Action
- [ ] Loader: bootboot
- [ ] Kernel: Minix Kernel
- [ ] Userland A: Minix Userland
- [ ] Userland B: Wayland + Stallion

## Task List
|Sl. | Task | Completed
|----|------------ | -------------
|1.|Using bootboot load the sample kernel on Pi4|Pending
|2.|Using bootboot load the sample kernel on Pi4|Pending
|3.|Compile Minix Kernel / Services using LLVM|Pending
|4.|Loading Minix Kernel using bootboot|Pending
|5.|Understand debugging Minix Kernel|Pending
|6.|USB Driver Support https://wiki.minix3.org/doku.php?id=ddekitusb|Pending

The following is copied from "MINIX 3 and Google Summer of Code 2018"

Project: ARM 64bit Support

MINIX 3 is currently 32-bit only. There are several task to be completed in order to have a full 64-bit port. Not all are expected to be achieved as a single project.
1. Enable toolchain support and definition of MINIX x86-64. This requires to adapt as needed LLVM and GNU binutils in order to define the OS support for this platform.
2. Compile the whole system in x86_64. At this point the executable code will be x86-64 binary, but it will still behave as a 32-bit OS, with only 32-bit address spaces per process.
3. Adapt the system types to 64bit. Some adaptations and correction will be needed in order to make sure that all the OS data types are adapted to 64bit.
4. Adapt the virtual memory manager of MINIX 3 to allow all the physical memory to be used. Add support to the memory manager to allow the use of all the physical RAM available, but with processes with 4GB address spaces still.
5. Adapt the memory model of processes to use 64bit ranges. Add support to the memory manager to allow the use of 64bit address

Project: rump

The rump anykernel essentially turns large parts of the NetBSD kernel (notably drivers, file systems and network stack) into portable, reusable components that can run anywhere. Adding rump support to MINIX would drastically boost the number of file systems and hardware peripherals supported by the operating system, vastly improving its usability on physical hardware along with tons of utilities like fs-utils or the rump server.

It has been proven that the rump components themselves will compile with little to no modification inside the MINIX tree, but there is no suitable hypercall implementation to build rump programs in user-space or glue code to run them inside the service or driver layer. We basically have a big heap of libraries without the needed foundations to make use of them.

Note: this is a really, really tough project for students looking for a really, really tough challenge. When in doubt, consider applying for another project.
