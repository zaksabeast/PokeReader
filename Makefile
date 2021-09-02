CRATE_NAME	= pkrd
TITLE_ID = 000401300000CB02

CARGO_BUILD_FLAGS = -Z build-std=core,alloc --target 3ds.json

RUST_OUT_DIR	=	target/3ds
RUST_RELEASE_DIR	=	$(RUST_OUT_DIR)/release
RUST_DEBUG_DIR	=	$(RUST_OUT_DIR)/debug

OUT_DIR	= out
RELEASE_DIR = $(OUT_DIR)/release
DEBUG_DIR = $(OUT_DIR)/debug
RELEASE_TITLE_DIR = $(RELEASE_DIR)/$(TITLE_ID)
DEBUG_TITLE_DIR = $(DEBUG_DIR)/$(TITLE_ID)

RELEASE_ELF	= $(RUST_RELEASE_DIR)/$(CRATE_NAME).elf
DEBUG_ELF	= $(RUST_DEBUG_DIR)/$(CRATE_NAME).elf

RELEASE_CXI	= $(RUST_RELEASE_DIR)/$(CRATE_NAME).cxi
DEBUG_CXI	= $(RUST_DEBUG_DIR)/$(CRATE_NAME).cxi

RELEASE_CIA	= $(RELEASE_DIR)/$(CRATE_NAME).cia
DEBUG_CIA	= $(DEBUG_DIR)/$(CRATE_NAME).cia

RELEASE_EXHEADER = $(RELEASE_TITLE_DIR)/exheader.bin
DEBUG_EXHEADER = $(DEBUG_TITLE_DIR)/exheader.bin

SOURCES = $(wildcard src/*.rs) $(wildcard src/**/*.rs) $(wildcard src/**/**/*.rs) $(wildcard src/**/**/**/*.rs)

.PHONY: all clean test docs lint

all: release debug test

docs:
	@cargo +nightly doc --open $(CARGO_BUILD_FLAGS)

# Nightly and unstable options prevent clippy from linting dependencies - https://github.com/rust-lang/rust-clippy/issues/1066
lint:
	@cargo +nightly clippy -Z unstable-options $(CARGO_BUILD_FLAGS)

test:
	@cargo +nightly test

release: $(RELEASE_EXHEADER) $(RELEASE_CIA)

debug: $(DEBUG_EXHEADER) $(DEBUG_CIA)

$(RELEASE_ELF) : $(SOURCES)
	@cargo +nightly build --release $(CARGO_BUILD_FLAGS)

$(DEBUG_ELF) : $(SOURCES)
	@cargo +nightly build $(CARGO_BUILD_FLAGS)

$(RELEASE_CIA) : $(RELEASE_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f cia -o $(RELEASE_CIA) -rsf pkrd.rsf -elf $(RELEASE_ELF)

$(DEBUG_CIA) : $(DEBUG_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f cia -o $(DEBUG_CIA) -rsf pkrd.rsf -elf $(DEBUG_ELF)

$(RELEASE_CXI) : $(RELEASE_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f ncch -rsf pkrd.rsf -o $@ -elf $<

$(DEBUG_CXI) : $(DEBUG_ELF)
	@mkdir -p $(DEBUG_DIR)
	@makerom -f ncch -rsf pkrd.rsf -o $@ -elf $<

$(RELEASE_EXHEADER) : $(RELEASE_CXI)
	@mkdir -p $(RELEASE_TITLE_DIR)
	@ctrtool --exefsdir=$(RELEASE_TITLE_DIR) --exheader=$@ $< > /dev/null
	@echo Built code and exheader

$(DEBUG_EXHEADER) : $(DEBUG_CXI)
	@mkdir -p $(DEBUG_TITLE_DIR)
	@ctrtool --exefsdir=$(DEBUG_TITLE_DIR) --exheader=$@ $< > /dev/null
	@echo Built code and exheader

clean:
	@rm -rf $(OUT_DIR)
	@cargo clean
