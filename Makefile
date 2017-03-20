#
# Makefile
# kotetuco, 2017
#

TARGET_ARCH := arm-none-eabi
BUILD_NAME := rom
BUILD_DIR := build

default:
	mkdir -p $(BUILD_DIR)
	make $(BUILD_DIR)/$(BUILD_NAME).mb

$(BUILD_DIR)/$(BUILD_NAME).mb: $(BUILD_DIR)/$(BUILD_NAME).elf
	$(TARGET_ARCH)-objcopy -O binary $(BUILD_DIR)/$(BUILD_NAME).elf \
	$(BUILD_DIR)/$(BUILD_NAME).mb

$(BUILD_DIR)/$(BUILD_NAME).elf: $(BUILD_DIR)/crt.o $(BUILD_DIR)/first.o \
rom.ld
	$(TARGET_ARCH)-ld -t -T rom.ld -o $(BUILD_DIR)/$(BUILD_NAME).elf \
	$(BUILD_DIR)/crt.o $(BUILD_DIR)/first.o -Map $(BUILD_DIR)/$(BUILD_DIR).map

$(BUILD_DIR)/first.o: first.rs $(BUILD_DIR)/libcore.rlib
	rustc --target=arm-none-eabi.json --crate-type=staticlib --emit=obj \
	-C lto -C opt-level=2 -C no-prepopulate-passes \
	-C relocation-model=static -Z verbose -Z no-landing-pads \
	-o $(BUILD_DIR)/first.o first.rs --extern core=$(BUILD_DIR)/libcore.rlib

$(BUILD_DIR)/libcore.rlib:\
$(shell rustc --print sysroot)/lib/rustlib/src/rust/src/libcore/lib.rs
	rustc --verbose --target=arm-none-eabi.json --crate-type=rlib \
	-C opt-level=2 -C no-prepopulate-passes -Z no-landing-pads \
	-o $(BUILD_DIR)/libcore.rlib \
	$(shell rustc --print sysroot)/lib/rustlib/src/rust/src/libcore/lib.rs

$(BUILD_DIR)/crt.o: crt.S
	$(TARGET_ARCH)-as crt.S -o $(BUILD_DIR)/crt.o

clean:
	rm -rf $(BUILD_DIR)
