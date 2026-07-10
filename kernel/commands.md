# Kernel ELF Info

```shell
readelf -l target/x86_64-unknown-none/debug/kernel
```

and 

```shell
readelf -d target/x86_64-unknown-none/debug/kernel
```

# Generate new kernel ELF
```shell
cargo clean
cargo build
```

# Build Kernel with nightly (standard)
```shell
cargo +nightly -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem build -Z build-std -Z json-target-spec
```

# copy kernel to iso folder
```shell
cp target/x86_64-unknown-none/debug/kernel iso/boot/kernel
```

# customized config kernel to iso folder
- this needs a x86_64-nucleus.json
- nightly is used for building
```shell
cp target/x86_64-nucleus/debug/kernel iso/boot/kernel
```

# Create ISO
```shell
xorriso -as mkisofs \
  -R -r -J \
  -b boot/limine/limine-bios-cd.bin \
  -no-emul-boot \
  -boot-load-size 4 \
  -boot-info-table \
  -hfsplus \
  -apm-block-size 2048 \
  --efi-boot boot/limine/limine-uefi-cd.bin \
  -efi-boot-part \
  --efi-boot-image \
  --protective-msdos-label \
  iso \
  -o nucleus.iso
```

# Install Limine's BIOS stages
```shell
~/osdev/limine-binary/limine bios-install nucleus.iso
```

# Boot ISO with qemu
```shell
qemu-system-x86_64 -cdrom nucleus.iso -m 512M
```

# Process
1. generate new kernel elf
2. copy kernel to iso folder
3. create iso
4. install bios stages
5. boot iso