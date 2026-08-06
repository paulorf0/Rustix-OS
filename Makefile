# Rustix — build and run automation
#
# Main targets:
#   make build      Compile the kernel and generate the BIOS/UEFI boot images
#   make run        Run the BIOS image in QEMU (default target)
#   make run-uefi   Run the UEFI image in QEMU with OVMF firmware
#   make debug      Run in BIOS mode, frozen at reset, waiting for GDB on :1234
#   make gdb        Attach GDB to a session started with `make debug`
#   make clean      Remove build artifacts of both the kernel and the builder

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

KERNEL_TARGET  := x86_64-unknown-none
HOST_TARGET    := x86_64-unknown-linux-gnu
TOOLCHAIN      := +nightly

# The builder currently hardcodes the "debug" profile when it looks for the
# kernel binary and when it writes the images, so this path is fixed for now.
OUT_DIR        := target/$(KERNEL_TARGET)/debug
KERNEL_BIN     := $(OUT_DIR)/rustix
BIOS_IMG       := $(OUT_DIR)/boot-bios.img
UEFI_IMG       := $(OUT_DIR)/boot-uefi.img

QEMU           := qemu-system-x86_64
QEMU_MEM       := 128M

# Flags shared by every QEMU invocation.
#   -serial stdio  routes the guest serial port to this terminal, which is the
#                  usual way to get kernel output before a real console exists
#   -no-reboot     makes a triple fault stop the VM instead of looping forever
QEMU_FLAGS     := -m $(QEMU_MEM) -serial stdio -no-reboot

# Extra flags appended to every run; override on the command line, e.g.
#   make run QEMU_EXTRA="-d int,cpu_reset"
QEMU_EXTRA     ?=

# ---------------------------------------------------------------------------
# OVMF (UEFI firmware) discovery
# ---------------------------------------------------------------------------
#
# Modern OVMF ships as two separate images: the firmware code (read-only) and
# an NVRAM variable store (read-write, holds boot entries). They are attached
# as two pflash devices rather than through -bios. Distributions disagree on
# where these files live, so try the common locations in order and let the
# user override with `make run-uefi OVMF_CODE=... OVMF_VARS=...`.

OVMF_CODE_CANDIDATES := \
	/usr/share/OVMF/OVMF_CODE.fd \
	/usr/share/edk2/ovmf/OVMF_CODE.fd \
	/usr/share/qemu/OVMF_CODE.fd \
	/usr/share/ovmf/OVMF.fd

OVMF_VARS_CANDIDATES := \
	/usr/share/OVMF/OVMF_VARS.fd \
	/usr/share/edk2/ovmf/OVMF_VARS.fd \
	/usr/share/qemu/OVMF_VARS.fd

OVMF_CODE ?= $(firstword $(wildcard $(OVMF_CODE_CANDIDATES)))
OVMF_VARS ?= $(firstword $(wildcard $(OVMF_VARS_CANDIDATES)))

# The variable store must be writable, so the pristine system file is copied
# into the build directory and QEMU is pointed at the copy.
OVMF_VARS_RW := $(OUT_DIR)/OVMF_VARS.fd

# ---------------------------------------------------------------------------
# Targets
# ---------------------------------------------------------------------------

.PHONY: all build run run-bios run-uefi debug debug-uefi gdb clean help

all: run

# The builder is a host program: it compiles the kernel for the bare-metal
# target and then wraps the resulting ELF into bootable disk images. It has to
# be built for the host target explicitly, because .cargo/config.toml sets
# x86_64-unknown-none as the default target for this workspace.
build:
	cargo $(TOOLCHAIN) run --manifest-path builder/Cargo.toml --target $(HOST_TARGET)

run: run-bios

run-bios: build
	$(QEMU) $(QEMU_FLAGS) $(QEMU_EXTRA) \
		-drive format=raw,file=$(BIOS_IMG)

run-uefi: build $(OVMF_VARS_RW)
	@test -n "$(OVMF_CODE)" || { \
		echo "OVMF firmware not found. Install it (Fedora: dnf install edk2-ovmf,"; \
		echo "Debian/Ubuntu: apt install ovmf) or pass OVMF_CODE=/path/to/OVMF_CODE.fd"; \
		exit 1; }
	$(QEMU) $(QEMU_FLAGS) $(QEMU_EXTRA) \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
		-drive if=pflash,format=raw,file=$(OVMF_VARS_RW) \
		-drive format=raw,file=$(UEFI_IMG)

# Copy the firmware variable store once, so UEFI boot entries survive between
# runs and the system-wide file is never written to.
$(OVMF_VARS_RW): | build
	@test -n "$(OVMF_VARS)" || { \
		echo "OVMF_VARS.fd not found. Pass OVMF_VARS=/path/to/OVMF_VARS.fd"; \
		exit 1; }
	cp $(OVMF_VARS) $@
	chmod u+w $@

# -s opens a GDB server on TCP :1234, -S halts the CPU before the first
# instruction so a breakpoint can be set before the bootloader runs.
debug: build
	$(QEMU) $(QEMU_FLAGS) $(QEMU_EXTRA) -s -S \
		-drive format=raw,file=$(BIOS_IMG)

debug-uefi: build $(OVMF_VARS_RW)
	$(QEMU) $(QEMU_FLAGS) $(QEMU_EXTRA) -s -S \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
		-drive if=pflash,format=raw,file=$(OVMF_VARS_RW) \
		-drive format=raw,file=$(UEFI_IMG)

# Symbols come from the kernel ELF, not from the disk image.
gdb:
	gdb $(KERNEL_BIN) -ex "target remote :1234"

clean:
	cargo clean
	cargo clean --manifest-path builder/Cargo.toml

help:
	@echo "build       compile kernel + generate boot-bios.img and boot-uefi.img"
	@echo "run         build, then boot the BIOS image in QEMU"
	@echo "run-uefi    build, then boot the UEFI image in QEMU with OVMF"
	@echo "debug       like run, but paused waiting for GDB on :1234"
	@echo "debug-uefi  like run-uefi, but paused waiting for GDB on :1234"
	@echo "gdb         attach GDB to a paused QEMU session"
	@echo "clean       remove kernel and builder build artifacts"
	@echo ""
	@echo "Overrides: QEMU_EXTRA=\"-d int\"  OVMF_CODE=...  OVMF_VARS=...  QEMU_MEM=256M"
