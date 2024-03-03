## SPDX-License-Identifier: MIT OR Apache-2.0
##
## Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>
# This file is part of the `rust-raspberrypi-OS-tutorials` project.
#
################################################################################

# this would build according to the rust target triple
# https://clang.llvm.org/docs/CrossCompilation.html#target-triple
TARGET            = aarch64-unknown-none-softfloat
KERNEL_BIN        = kernel8.img
QEMU_BINARY       = qemu-system-aarch64
QEMU_MACHINE_TYPE = raspi3
QEMU_RELEASE_ARGS = -d in_asm -display none
RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53
LD_SCRIPT_PATH    = src/bsp/raspberrypi/
LD_SCRIPT_NAME    = linker.ld
KERNEL_LINKER_SCRIPT = $(LD_SCRIPT_PATH)$(LD_SCRIPT_NAME)
RUSTFLAGS = $(RUSTC_MISC_ARGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

# build my kernel
KERNEL_ELF = target/$(TARGET)/release/kernel

COMPILER_ARGS = --target=$(TARGET) --release

$(KERNEL_ELF): 
	@RUSTFLAGS="$(RUSTFLAGS)" cargo rustc $(COMPILER_ARGS)

$(KERNEL_BIN): $(KERNEL_ELF)
	rust-objcopy -O binary $(KERNEL_ELF) $(KERNEL_BIN)

all: $(KERNEL_BIN)

qemu: $(KERNEL_BIN)
	qemu-system-aarch64 -M raspi3b -kernel $(KERNEL_BIN) -d in_asm -display none 

clean:
	rm -rf target $(KERNEL_BIN)