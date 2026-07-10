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

# copy kernel to iso folder
```shell
cp target/x86_64-unknown-none/debug/kernel iso/boot/kernel
```

# Build ISO
```shell
~/osdev/limine-binary/limine bios-install nucleus.iso
```

# Boot ISO with qemu
```shell
qemu-system-x86_64 -cdrom nucleus.iso -m 512M
```