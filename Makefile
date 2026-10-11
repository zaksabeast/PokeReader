.SUFFIXES:

ifeq ($(strip $(DEVKITARM)),)
$(error "Please set DEVKITARM in your environment. export DEVKITARM=<path to>devkitARM")
endif

TOPDIR ?= $(CURDIR)
include $(DEVKITARM)/3ds_rules

PLGINFO := PokeReader.plgInfo

BUILD   := build

#---------------------------------------------------------------------------------
# Rust configuration
#---------------------------------------------------------------------------------
RUST_TARGET := armv6k-nintendo-3ds
RUST_PROFILE := release
RUST_DIR := $(TOPDIR)/target/$(RUST_TARGET)/$(RUST_PROFILE)
RUST_LIB := $(RUST_DIR)/libpokereader.a

#---------------------------------------------------------------------------------
# options for code generation
#---------------------------------------------------------------------------------
ARCH := -march=armv6k -mtune=mpcore -mfloat-abi=hard -mtp=soft

LDFLAGS := -T $(TOPDIR)/3gx.ld $(ARCH) -Os -Wl,-u,_start \
           -Wl,--gc-sections,--strip-discarded,--strip-debug,-z,noexecstack \
           -L$(RUST_DIR)

LIBS := -lpokereader -lctru
LIBDIRS := $(CTRULIB) $(PORTLIBS)

#---------------------------------------------------------------------------------
ifneq ($(BUILD),$(notdir $(CURDIR)))
#---------------------------------------------------------------------------------

export OUTPUT := default
export TOPDIR := $(CURDIR)

export LD := $(CXX)

export LIBPATHS := $(foreach dir,$(LIBDIRS),-L $(dir)/lib)

.PHONY: all rust clean re lint test format

#---------------------------------------------------------------------------------
# Build Rust first.
#
# Cargo handles Rust's dependency tracking, so we deliberately make this
# phony. Cargo will do nothing when the Rust sources are already up to date.
#---------------------------------------------------------------------------------
all: $(BUILD)
	@mkdir -p out
	@cp $(BUILD)/$(OUTPUT).3gx out/default.3gx

rust:
	@echo "==> Building Rust..."
	@cargo build --release -Z build-std=core,alloc --target $(RUST_TARGET)

#---------------------------------------------------------------------------------
# Enter the normal devkitARM build directory.
#---------------------------------------------------------------------------------
$(BUILD):
	@mkdir -p $@
	@$(MAKE) --no-print-directory -C $(BUILD) -f $(CURDIR)/Makefile

#---------------------------------------------------------------------------------
# Make sure Rust has been built before the normal build starts.
#---------------------------------------------------------------------------------
$(BUILD): rust

#---------------------------------------------------------------------------------
clean:
	@echo "==> Cleaning..."
	@cargo clean
	@rm -rf $(BUILD) out

re: clean all

lint:
	@cargo clippy --release -Z build-std=core,alloc --target $(RUST_TARGET)

test:
	@cargo test

format:
	@cargo fmt --all

#---------------------------------------------------------------------------------

else

#---------------------------------------------------------------------------------
# Main targets
#
# Rust is built by the parent Make invocation before this recursive make runs.
# The explicit Rust library dependency also ensures the linker knows that the
# archive is an input to the final binary.
#---------------------------------------------------------------------------------
$(OUTPUT).3gx: $(OUTPUT).elf $(RUST_LIB)

$(OUTPUT).elf: $(RUST_LIB)

#---------------------------------------------------------------------------------
.PRECIOUS: %.elf

%.3gx: %.elf
	@echo creating $(notdir $@)
	@3gxtool -s $(word 1, $^) $(TOPDIR)/$(PLGINFO) $@

#---------------------------------------------------------------------------------
endif
