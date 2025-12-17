# ===========================
# Project Configuration
# ===========================
BINARY_NAME := ezc
TARGET_DIR  := target
RELEASE     := release

# ===========================
# Supported Targets
# ===========================
LINUX_GNU        := x86_64-unknown-linux-gnu
LINUX_MUSL       := x86_64-unknown-linux-musl
MACOS_ARM        := aarch64-apple-darwin
MACOS_INTEL      := x86_64-apple-darwin
WINDOWS_GNU      := x86_64-pc-windows-gnu

TARGETS := \
	$(LINUX_GNU) \
	$(LINUX_MUSL) \
	$(MACOS_ARM) \
	$(MACOS_INTEL) \
	$(WINDOWS_GNU)

# ===========================
# CPU-Specific Optimizations
# ===========================
RUSTFLAGS_NATIVE := -C target-cpu=native

# ===========================
# Default Target
# ===========================
.PHONY: all
all: native

# ===========================
# Native Optimized Build
# ===========================
.PHONY: native
native:
	RUSTFLAGS="$(RUSTFLAGS_NATIVE)" \
	cargo build --release
	@echo "✅ Native optimized binary built"

# ===========================
# Cross Compilation
# ===========================
define BUILD_TARGET
.PHONY: build-$(1)
build-$(1):
	rustup target add $(1)
	cargo build --release --target $(1)
	@echo "✅ Built for $(1)"
endef

$(foreach t,$(TARGETS),$(eval $(call BUILD_TARGET,$(t))))

# ===========================
# Build All Platforms
# ===========================
.PHONY: build-all
build-all: $(addprefix build-,$(TARGETS))

# ===========================
# Strip Binaries (Extra Size Reduction)
# ===========================
.PHONY: strip
strip:
	@echo "🔧 Stripping binaries..."
	strip target/**/release/$(BINARY_NAME)* || true

# ===========================
# Clean
# ===========================
.PHONY: clean
clean:
	cargo clean

# ===========================
# Size Report
# ===========================
.PHONY: size
size:
	@echo "📦 Binary sizes:"
	@ls -lh target/**/release/$(BINARY_NAME)* || true
