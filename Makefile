arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-rustbucket_os
rust_os := target/$(target)/debug/librustbucket_os.a

linker_script := kernel/arch/$(arch)/linker.ld
grub_cfg := kernel/arch/$(arch)/grub.cfg

assembly_boot_files := $(wildcard kernel/arch/$(arch)/boot/*.asm)
assembly_boot_o_files := $(patsubst kernel/arch/$(arch)/boot/%.asm, \
  build/arch/$(arch)/boot/%.o, $(assembly_boot_files))

assembly_int_files := $(wildcard kernel/arch/$(arch)/int/*.asm)
assembly_int_o_files := $(patsubst kernel/arch/$(arch)/int/%.asm, \
  build/arch/$(arch)/int/%.o, $(assembly_int_files))

.PHONY: all clean run run-log run-test run-test-hidden iso kernel

all: $(kernel) $(iso)

clean:
	@rm -r build

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

run-log: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -serial mon:stdio

run-test: $(iso)
	qemu-system-x86_64 -cdrom $(iso) \
	-serial mon:stdio \
	-device isa-debug-exit,iobase=0xf4,iosize=0x04 \

run-test-hidden: $(iso)
	qemu-system-x86_64 -cdrom $(iso) \
	-serial mon:stdio \
	-device isa-debug-exit,iobase=0xf4,iosize=0x04
	-display none

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_boot_o_files) $(assembly_int_o_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) \
	$(rust_os) --start-group $(assembly_int_o_files) $(assembly_boot_o_files) $(rust_os) --end-group 

kernel:
	@RUST_TARGET_PATH="$(pwd)" xargo build --target $(target)

# compile assembly files
build/arch/$(arch)/boot/%.o: kernel/arch/$(arch)/boot/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@

build/arch/$(arch)/int/%.o: kernel/arch/$(arch)/int/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@
