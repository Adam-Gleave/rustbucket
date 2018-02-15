arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-rustbucket_os
rust_os := target/$(target)/debug/librustbucket_os.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

assembly_boot_files := $(wildcard src/arch/$(arch)/boot/*.asm)
assembly_boot_o_files := $(patsubst src/arch/$(arch)/boot/%.asm, \
  build/arch/$(arch)/boot/%.o, $(assembly_boot_files))

assembly_int_files := $(wildcard src/arch/$(arch)/interrupts/*.asm)
assembly_int_o_files := $(patsubst src/arch/$(arch)/interrupts/%.asm, \
  build/arch/$(arch)/interrupts/%.o, $(assembly_int_files))

.PHONY: all clean run iso kernel

all: $(kernel) $(iso)

clean:
	@rm -r build

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

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
	@xargo build --target $(target)

# compile assembly files
build/arch/$(arch)/boot/%.o: src/arch/$(arch)/boot/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@

build/arch/$(arch)/interrupts/%.o: src/arch/$(arch)/interrupts/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@
