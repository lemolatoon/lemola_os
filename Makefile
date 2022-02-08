build:
	cargo build --verbose

target/x86_64-unknown-uefi/debug/uefi_lemola_os.efi: build
	mkdir -p mnt/EFI/BOOT && \
	cp target/x86_64-unknown-uefi/debug/uefi_lemola_os.efi mnt/EFI/BOOT/BOOTX64.EFI 


run: target/x86_64-unknown-uefi/debug/uefi_lemola_os.efi
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,readonly,file=ovmf/OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=ovmf/lemola_ovmf_vars.fd  \
		-drive media=disk,format=raw,file=fat:rw:mnt \
