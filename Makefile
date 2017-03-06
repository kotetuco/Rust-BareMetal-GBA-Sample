#
# Makefile
# kotetuco, 2017
#

TARGET_ARCH := arm-none-eabi
TARGET_ARCH_RUST := $(TARGET_ARCH)
BUILD_NAME := rom
BUILD_DIR := build
BUILD_MODE := debug

default:
	mkdir -p $(BUILD_DIR)
	make $(BUILD_DIR)/$(BUILD_NAME).mb

$(BUILD_DIR)/$(BUILD_NAME).mb: $(BUILD_DIR)/$(BUILD_NAME).elf Makefile
	$(TARGET_ARCH)-objcopy -O binary $(BUILD_DIR)/$(BUILD_NAME).elf $(BUILD_DIR)/$(BUILD_NAME).mb

$(BUILD_DIR)/$(BUILD_NAME).elf: $(BUILD_DIR)/crt.o rom.ld target/$(TARGET_ARCH_RUST)/$(BUILD_MODE)/librust_basemetal_gba.a
	$(TARGET_ARCH)-ld --gc-sections -t -T rom.ld -o $(BUILD_DIR)/$(BUILD_NAME).elf $(BUILD_DIR)/crt.o --library-path=target/$(TARGET_ARCH_RUST)/$(BUILD_MODE) -lrust_basemetal_gba -Map $(BUILD_DIR)/$(BUILD_NAME).map

target/$(TARGET_ARCH_RUST)/$(BUILD_MODE)/librust_basemetal_gba.a: $(TARGET_ARCH_RUST).json Cargo.toml src/*.rs
	RUST_TARGET_PATH=$(PWD) rustup run nightly `which xargo` build -v --target=$(TARGET_ARCH_RUST)

$(BUILD_DIR)/crt.o: crt.S
	$(TARGET_ARCH)-as crt.S -o $(BUILD_DIR)/crt.o

clean:
	rm -rf $(BUILD_DIR)
	xargo clean
