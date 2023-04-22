LD = riscv64-unknown-elf-ld
OBJDUMP = riscv64-unknown-elf-objdump

.PHONY: all clean qemu
all: kernel
	@qemu-system-riscv64 -S -s -machine virt -nographic -bios $< &
	@RUST_GDB=riscv64-unknown-elf-gdb rust-gdb -q --tui \
		-ex "target remote :1234" \
		-ex "layout split" \
		-ex "tb kernel_main" \
		-ex "c" $<
	@killall qemu-system-riscv64

qemu: kernel
	@qemu-system-riscv64 -machine virt -nographic -bios $<

kernel: kernel.ld FORCE
	@cargo build -Z unstable-options --out-dir .
	@$(LD) -T $< -o $@ lib$@.a
	@rm -f lib$@.a

clean:
	@cargo clean
	@rm -rf kernel

FORCE: