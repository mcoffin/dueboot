ARDUINO=/Applications/Arduino.app/Contents/Java
RUSTC=rustc
LLC=/Users/mcoffin/workspace/rust/build/x86_64-apple-darwin/llvm/Release+Asserts/bin/llc
CORE_SRC=/Users/mcoffin/workspace/rust/src/libcore/lib.rs
RUSTC_TARGET=arm-unknown-linux-gnueabi
TARGET=arm-non-eabi
CC=$(ARDUINO)/hardware/tools/gcc-arm-none-eabi-4.8.3-2014q1/bin/arm-none-eabi-gcc
CXX=$(ARDUINO)/hardware/tools/gcc-arm-none-eabi-4.8.3-2014q1/bin/arm-none-eabi-g++
OBJCOPY=$(ARDUINO)/hardware/tools/gcc-arm-none-eabi-4.8.3-2014q1/bin/arm-none-eabi-objcopy
SRC=main.rs

MAIN_CRATE_NAME=$(shell $(RUSTC) --print crate-name $(SRC))
CORE_LIB=$(shell $(RUSTC) --print file-names $(CORE_SRC))

CRATES=main core
CRATE_FILES=$(CRATES:=.o)

RUSTC_FLAGS=--target=$(RUSTC_TARGET) -A dead_code -A non_snake_case -C no-stack-check
LLC_FLAGS=-march=thumb -mattr=+thumb2 -mcpu=cortex-m3 --float-abi=soft -asm-verbose

all: boot.bin

clean:
	rm *.ll *.s *.o *.elf *.bin *.rlib

$(CORE_LIB): $(CORE_SRC)
	$(RUSTC) $(RUSTC_FLAGS) --emit=llvm-ir,link $(CORE_SRC)

%.s: %.ll
	sed -i .1 's/$(RUSTC_TARGET)/$(TARGET)/g' $<
	$(LLC) $(LLC_FLAGS) $< -o=$@

%.ll: %.rs
	$(RUSTC) $(RUSTC_FLAGS) -L . --emit=llvm-ir $<

%.o: %.s
	$(CC) $(CFLAGS) $< -o $@

boot.elf: $(CORE_LIB) $(CRATE_FILES)
	$(CXX) $(LDFLAGS) $(CRATE_FILES) -o $@

%.bin: %.elf
	$(OBJCOPY) -O binary $< $@
