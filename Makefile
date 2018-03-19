# Configuration that will be passed to sub-make

# Name of the kernel module
export KERNEL_MODULE := roulette

# Path to Linux kernel headers
export KERNEL_BUILD_PATH := /lib/modules/$(shell uname -r)/build

# Find all C and Rust source files
export C_FILES := $(shell find src/ -type f -name "*.c")
export RUST_FILES := $(shell find src/ -type f -name "*.rs") Cargo.toml Cargo.lock

# Define the architecture; this will be used for the LLVM target specification
export UTS_MACHINE = x86_64

# The Rust compiler and cross-compiler
export CARGO=cargo
export XARGO=xargo

# A JSON file specifying an LLVM target
export LLVM_TARGET_SPEC=$(UTS_MACHINE)-unknown-none-gnu.json

# Top-level project directory
export BASE_DIR := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

# The build directory
export BUILD_DIR := build

# The Makefile that is copied to $(BUILD_DIR)
export KBUILD := kbuild.mk

all: $(BUILD_DIR)/Makefile Makefile
# The kbuild makefile has been copied to $(BUILD_DIR), so now we can invoke kbuild from
# the kernel headers.
	$(MAKE) -C "$(KERNEL_BUILD_PATH)" M="$(BASE_DIR)/$(BUILD_DIR)" modules

$(BUILD_DIR)/Makefile : $(KBUILD)
	@mkdir -p "${BUILD_DIR}/src"
	cp "$(KBUILD)" "$(BUILD_DIR)/Makefile"

clean:
# cleanup is really simple, we just blow away the $(BUILD_DIR) and call `xargo clean`
	rm -rf "$(BUILD_DIR)"
	xargo clean
