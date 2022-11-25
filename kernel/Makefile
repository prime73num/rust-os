# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm
BOOTLOADER := ./bootloader/rustsbi-qemu.bin
KERNEL_ENTRY_PA := 0x80200000

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

build: 
	@cargo build $(MODE_ARG)

dump: build
	rust-objcopy --binary-architecture=riscv64 ${KERNEL_ELF} --strip-all -D ${KERNEL_BIN}

run: dump
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

clean:
	cargo clean
