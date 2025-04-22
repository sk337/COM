.phony: all clean

NASM = nasm
NASM_FLAGS = -f bin

# asm files can end with s,as,asm
ASM_FILES = $(wildcard *.s) $(wildcard *.as) $(wildcard *.asm)

# compiled asm files with .com extension into build directory
COM_FILES = $(patsubst %.s,build/%.com,$(patsubst %.as,build/%.com,$(patsubst %.asm,build/%.com,$(ASM_FILES))))

all: $(COM_FILES)

build/%.com: %.s build
	@mkdir -p build
	$(NASM) $(NASM_FLAGS) $< -o $@

build/%.com: %.as build
	@mkdir -p build
	$(NASM) $(NASM_FLAGS) $< -o $@

build/%.com: %.asm build
	$(NASM) $(NASM_FLAGS) $< -o $@

build:
	@mkdir -p build

clean:
	rm -rf build