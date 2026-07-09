
# Let's start from power-on

Imagine a real computer.

You press the power button.

```text
Power
   │
   ▼
CPU resets
   │
   ▼
Firmware starts
```

Now, depending on the machine, the firmware is one of two things:

```
Old PC
    BIOS

Modern PC
    UEFI
```

Notice something.

At this point **your kernel does not exist**.

The firmware has absolutely no idea what Nucleus is.

---

# Firmware's Job

Firmware only knows how to start a bootloader.

Think of firmware as saying:

> "I know how to find and launch a boot program."

It does **not** know:

* ELF
* Rust
* `_start`
* Your linker script

Those are someone else's responsibility.

---

# So who understands kernels?

That's Limine.

```text
Firmware
    │
    ▼
Limine
    │
    ▼
Kernel
```

---

# BIOS path

Let's take BIOS first because it's older.

BIOS understands disks at a **very low level**.

Think:

```
Sector 0
Sector 1
Sector 2
...
```

Not files.

Not folders.

Not filenames.

Just sectors.

So BIOS cannot say

> "Open `/boot/kernel`."

It literally doesn't know what `/boot` means.

---

## Then how does BIOS start Limine?

This is where these files come in.

### `limine-bios-cd.bin`

This is the **first stage** boot code for a BIOS CD.

Its job is tiny.

```
BIOS
    │
    ▼
limine-bios-cd.bin
```

It gets control first.

It cannot yet load your kernel.

It simply knows how to continue booting Limine.

---

### `limine-bios.sys`

Think of this as the **main BIOS bootloader**.

```
BIOS
   │
   ▼
limine-bios-cd.bin
   │
   ▼
limine-bios.sys
   │
   ▼
Kernel
```

The first stage is intentionally tiny because BIOS gives very little room at the beginning of the boot media.

This is a common pattern in bootloaders:

```
Tiny stage
      ↓
Larger stage
      ↓
Kernel
```

---

# UEFI is completely different

UEFI is much more modern.

Unlike BIOS...

UEFI **understands filesystems**.

It can literally open files.

That changes everything.

Instead of loading mysterious sectors...

UEFI simply says

> "Open this executable."

---

## That's why we copied

```
BOOTX64.EFI
```

This is an ordinary UEFI executable.

Think of it like

```
Windows

explorer.exe
```

except instead of Windows launching it...

UEFI launches it.

---

# Why this path?

You noticed

```
EFI/
└── BOOT/
    └── BOOTX64.EFI
```

Why not somewhere else?

Because the UEFI specification says:

> If removable media is inserted...

look for

```
EFI/BOOT/BOOTX64.EFI
```

That filename is standardized.

The firmware expects it.

Not Limine.

---

# Now here's the beautiful part

Both BIOS and UEFI eventually arrive here

```
                BIOS
                  │
                  ▼
        limine-bios.sys

                UEFI
                  │
                  ▼
          BOOTX64.EFI

                  │
                  ▼
              Limine Core
```

Now both paths merge.

From here onward...

everything is identical.

---

# Then comes `limine.conf`

Remember your earlier answer?

You correctly reasoned

> Limine needs a config file.

Exactly.

Limine starts.

Then reads

```
limine.conf
```

Inside it we wrote

```
protocol: limine

path: boot():/boot/kernel
```

Meaning

```
Load this ELF.
```

---

# Then finally...

Limine opens

```
/boot/kernel
```

Not because firmware told it.

Because **its configuration** told it.

Then...

```
Read ELF Header

↓

Read Entry Point

↓

Jump
```

Everything we've spent the last several weeks learning suddenly becomes relevant.

---

# So why isn't the kernel inside EFI?

This is the question I wanted to ask you earlier.

Think about responsibilities.

```
Firmware
```

needs

```
BOOTX64.EFI
```

to start.

Once Limine is running...

does firmware matter anymore?

No.

Now Limine is in charge.

So Limine can load

```
/boot/kernel
```

or

```
/system/kernel
```

or

```
/nucleus/kernel
```

or anywhere else.

The firmware never sees it.

That's why the kernel is **not** stored under `EFI/BOOT`.

---

# The architecture

This is the mental picture I want you to keep forever.

```text
                   Power
                     │
                     ▼
               CPU Reset
                     │
                     ▼
          Firmware (BIOS/UEFI)
             │             │
             ▼             ▼
   limine-bios...    BOOTX64.EFI
              \       /
               \     /
                ▼   ▼
                 Limine
                    │
             Reads limine.conf
                    │
                    ▼
            Opens /boot/kernel
                    │
            Reads ELF Header
                    │
        Reads Entry Point Address
                    │
                    ▼
             Jumps into Nucleus
```

---
