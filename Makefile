.PHONY: all draco redox qemu help

all: draco redox

draco:
	cd draco/draco_voice && cargo build
	cd draco/draco_vision && cargo build

redox:
	cd redox && export M4=/opt/homebrew/opt/m4/bin/m4 && export PATH="/opt/homebrew/bin:$$PATH" && make all CONFIG_NAME=draco ARCH=x86_64 PODMAN_BUILD=0

qemu:
	cd redox && export M4=/opt/homebrew/opt/m4/bin/m4 && export PATH="/opt/homebrew/bin:$$PATH" && make qemu CONFIG_NAME=draco ARCH=x86_64 PODMAN_BUILD=0

help:
	@echo "Draco-OS Build System"
	@echo ""
	@echo "Usage:"
	@echo "  make draco   - Build Draco AI services (voice & vision)"
	@echo "  make redox   - Build Redox microkernel and system"
	@echo "  make qemu    - Launch Draco-OS in QEMU"
	@echo "  make all     - Build everything"
