BSP ?= rpi3

TARGET					=	aarch64-unknown-none-softfloat
KERNEL_BIN				=	kernel8.img
QEMU_BINARY				=	qemu-system-aarch64
QEMU_RELEASE_ARGS		=	-serial stdio -display none#-d in_asm -display none
OBJDUMP_BINARY			=	aarch64-none-elf-objdump
NM_BINARY				=	aarch64-none-elf-nm
READELF_BINARY			=	aarch64-none-elf-readelf
LINKER_FILE				=	src/bsp/raspberrypi/link.ld
ifeq ($(BSP), rpi3)
	QEMU_MACHINE_TYPE	=	raspi3
	RUSTC_MISC_ARGS		=	-C target-cpu=cortex-a53
else ifeq($(BSP), rpi4)
	QEMU_MACHINE_TYPE	=	raspi4
	RUSTC_MISC_ARGS		=	-C target-cpu=cortex-a72
endif

export LINKER_FILE

KERNEL_ELF	=	target/${TARGET}/release/kernel

##-----------------------------------------------------------------------------
## Commands
##-----------------------------------------------------------------------------
RUSTFLAGS			=	-C link-args=-T${LINKER_FILE} ${RUSTC_MISC_ARGS}
RUSTFLAGS_PEDANTIC	=	${RUSTFLAGS} -D warnings -D missing_docs

FEATURES		=	--features bsp_${BSP}
COMPILER_ARGS	=	--target=${TARGET} ${FEATURES} --release

RUSTC_CMD		=	cargo rustc ${COMPILER_ARGS}
DOC_CMD			=	cargo doc ${COMPILER_ARGS}
CLIPPY_CMD		=	cargo clippy ${COMPILER_ARGS}
CHECK_CMD		=	cargo check ${COMPILER_ARGS}
OBJCOPY_CMD		=	rust-objcopy --strip-all -O binary
EXEC_QEMU		=	${QEMU_BINARY} -M ${QEMU_MACHINE_TYPE}
EXEC_TEST		=	ruby ../common/tests/dispatch.rb

##------------------------------------------------------------------------------
## Docker Commands
##-----------------------------------------------------------------------------
DOCKER_IMAGE	=	rustembedded/osdev-utils
DOCKER_CMD		=	docker run -t --rm -v $(shell pwd):/work/tutorial -w /work/tutorial
DOCKER_CMN_DIR	=	-v $(shell pwd)/../common:/work/common
DOCKER_CMD_I	=	$(DOCKER_CMD) -i
DOCKER_QEMU		=	$(DOCKER_CMD_I) $(DOCKER_IMAGE)
DOCKER_TOOLS	=	$(DOCKER_CMD) $(DOCKER_IMAGE)
DOCKER_TEST		=	$(DOCKER_CMD) $(DOCKER_CMN_DIR)
##-----------------------------------------------------------------------------
## Targets
##-----------------------------------------------------------------------------
.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) doc qemu clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	echo "\nCompiling kernel - $(BSP)"
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

doc:
	echo "\nGenerating docs"
	@$(DOC_CMD) --document-private-items --open

clippy:
	@RUSTFLAGS = "$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

check:
	@RUSTFLAGS = "$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json

clean:
	rm -rf target $(KERNEL_BIN)

objdump: $(KERNEL_ELF)
	@$(DOCKER_TOOLS) $(OBJDUMP_BINARY) --disassemble --demangle --section .text \
					--section .rodata --section .got $(KERNEL_ELF) | rustfilt

readelf:	$(KERNEL_ELF)
	echo "\nLaunching readelf...Not Supported yet!"

qemu: #rust-analyzer
	@$(DOCKER_QEMU) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

nm:
	@$(DOCKER_TOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt