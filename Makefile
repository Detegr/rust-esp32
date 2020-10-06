RUST_XTENSA_PATH := $(shell pwd)/../rust-xtensa
RUST_XTENSA_BINARY := ${RUST_XTENSA_PATH}/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
RUST_SRC_PATH := ${RUST_XTENSA_PATH}/library
XTENSA_BIN_PATH := $(shell pwd)/../esp/xtensa-esp32-elf/bin

DEV := /dev/ttyUSB0

all:
	RUST_BACKTRACE=1 RUSTC=${RUST_XTENSA_BINARY} XARGO_RUST_SRC=${RUST_SRC_PATH} PATH=${PATH}:${XTENSA_BIN_PATH} cargo xbuild --release 

flash:
	cargo espflash --chip esp32 --speed 460800 --release ${DEV}

.PHONY: all flash
