build: kernel.elf uefi_lemola_os.efi 

uefi_lemola_os.efi:
	cd bootloader && \
	cargo build && \
	cd ..

kernel.elf:
	cd kernel && \
	cargo build && \
	cd ..

_kernel.elf:
	cd kernel/tmp && \
	clang++ -O2 -Wall -g --target=x86_64-elf -ffreestanding -mno-red-zone \
	-fno-exceptions -fno-rtti -std=c++17 -c main.cpp && \
	ld.lld --entry KernelMain -z norelro --image-base 0x100000 -static \
	-o kernel.elf main.o && \
	cp kernel.elf .. && \
	cd ../..

clippy:
	cd bootloader && \
	cargo clippy && \
	cd ..

ready: uefi_lemola_os.efi kernel.elf
	mkdir -p mnt/EFI/BOOT && \
	cp bootloader/target/x86_64-unknown-uefi/debug/uefi_lemola_os.efi mnt/EFI/BOOT/BOOTX64.EFI && \
	cp kernel/kernel.elf mnt/kernel.elf

kernel/kernel.elf: build


run: ready
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,readonly,file=ovmf/OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=ovmf/lemola_ovmf_vars.fd  \
		-drive media=disk,format=raw,file=fat:rw:mnt \
		-monitor stdio

fix:
	# git checkout 56685e2ed7b72ed1b325402b37fb5a04fff3ace7 ovmf/lemola_ovmf_vars.fd
	# git checkout 1bf96c63e2b090910f9988fa6afd1ccf01b51a59 ovmf/lemola_ovmf_vars.fd
	git checkout 90a30e645418ac235b63ee9f1717babca336231b ovmf/lemola_ovmf_vars.fd


.PHONY: docker_build
docker_build:
	docker build -t lemola_os .devcontainer/

.PHONY: docker_run
docker_run:
	docker run -v `pwd`:/lemola_os -it lemola_os:latest /bin/bash
