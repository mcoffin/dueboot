ARDUINO=/Applications/Arduino.app/Contents/Java
RUSTC=rustc
LLC=/Users/mcoffin/workspace/rust/build/x86_64-apple-darwin/llvm/Release+Asserts/bin/llc
CORE_SRC=/Users/mcoffin/workspace/rust/src/libcore/lib.rs
RUSTC_TARGET=arm-unknown-linux-gnueabi
TARGET=arm-non-eabi
AR=$(ARDUINO)/hardware/tools/gcc-arm-none-eabi-4.8.3-2014q1/bin/arm-none-eabi-ar
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

BOOT_C_SRCS=$(shell find $(ARDUINO)/hardware/arduino/sam/cores/arduino -name "*.c")
BOOT_CPP_SRCS=$(shell find $(ARDUINO)/hardware/arduino/sam/cores/arduino -name "*.cpp") $(ARDUINO)/hardware/arduino/sam/variants/arduino_due_x/variant.cpp
BOOT_INCLUDES=$(ARDUINO)/hardware/arduino/sam/system/libsam $(ARDUINO)/hardware/arduino/sam/system/CMSIS/CMSIS/Include $(ARDUINO)/hardware/arduino/sam/cores/arduino $(ARDUINO)/hardware/arduino/sam/system/CMSIS/Device/ATMEL $(ARDUINO)/hardware/arduino/sam/variants/arduino_due_x

ARDUINO_OBJS=$(BOOT_C_SRCS:.c=.o) $(BOOT_CPP_SRCS:.cpp=.o)

CFLAGS=-c -g -Os -w -ffunction-sections -fdata-sections -nostdlib --param max-inline-insns-single=500 -Dprintf=iprintf -mcpu=cortex-m3 -DF_CPU=84000000L -DARDUINO=155 -DARDUINO_SAM_DUE -DARDUINO_ARCH_SAM -D__SAM3X8E__ -mthumb -DUSB_VID=0x2341 -DUSB_PID=0x003e -DUSBCON $(foreach d, $(BOOT_INCLUDES), -I$d)

CXXFLAGS=$(CFLAGS) -fno-rtti -fno-exceptions

LDFLAGS=-Os -Wl,--gc-sections -mcpu=cortex-m3 -T$(ARDUINO)/hardware/arduino/sam/variants/arduino_due_x/linker_scripts/gcc/flash.ld -lm -lgcc -mthumb -Wl,--cref -Wl,--entry=Reset_Handler -Wl,--unresolved-symbols=report-all -Wl,--warn-common -Wl,--warn-section-align -Wl,--warn-unresolved-symbols -Wl,--start-group

all: boot.bin

flash: boot.bin
	$(ARDUINO)/hardware/tools/bossac -U false -e -w -v -b $< -R

clean:
	rm *.ll *.ll.1 *.s *.o *.elf *.bin *.rlib *.a

$(CORE_LIB): $(CORE_SRC)
	$(RUSTC) $(RUSTC_FLAGS) --emit=link $(CORE_SRC)

core.ll: $(CORE_SRC)
	$(RUSTC) $(RUSTC_FLAGS) --emit=llvm-ir $(CORE_SRC)

%.s: %.ll
	sed -i .1 's/$(RUSTC_TARGET)/$(TARGET)/g' $<
	$(LLC) $(LLC_FLAGS) $< -o=$@

%.ll: %.rs
	$(RUSTC) $(RUSTC_FLAGS) -L . --emit=llvm-ir $<

%.o: %.s
	$(CC) $(CFLAGS) $< -o $@

boot.elf: $(CORE_LIB) $(CRATE_FILES) arduino.a $(ARDUINO)/hardware/arduino/sam/variants/arduino_due_x/libsam_sam3x8e_gcc_rel.a
	$(CXX) $(LDFLAGS) $(CRATE_FILES) arduino.a $(ARDUINO)/hardware/arduino/sam/variants/arduino_due_x/libsam_sam3x8e_gcc_rel.a -Wl,--end-group -o $@

%.bin: %.elf
	$(OBJCOPY) -O binary $< $@

%.o: %.c
	$(CC) $(CFLAGS) $< -o $@

%.o: %.cpp
	$(CXX) $(CXXFLAGS) $< -o $@

arduino.a: $(ARDUINO_OBJS)
	$(AR) rcs $@ $(ARDUINO_OBJS)
