build: kernel.elf uefi_lemola_os.efi 

uefi_lemola_os.efi:
	cd bootloader && \
	cargo +nightly build && \
	cd ..

kernel.elf:
	cd kernel && \
	rustup run nightly cargo build && \
	cd ..

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
	git checkout 56685e2ed7b72ed1b325402b37fb5a04fff3ace7 ovmf/lemola_ovmf_vars.fd