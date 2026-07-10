Excellent idea. Before we build the PMM, let's consolidate what we've learned. Think of this as **Nucleus Notes #1: Framebuffer**.

---

# Nucleus Notes #1 — Framebuffer

## 1. What is a framebuffer?

A framebuffer is simply **a contiguous block of RAM that represents the pixels currently displayed on the monitor**.

Instead of saying:

> "Print character A"

you say:

> "Pixel (100, 50) should be blue."

The monitor (or virtual GPU in QEMU) repeatedly reads this memory and converts it into the image you see.

```
Kernel
   │
   ▼
Framebuffer Memory
   │
   ▼
GPU / Display Controller
   │
   ▼
Monitor
```

The kernel doesn't "draw" directly on the monitor. It modifies memory.

---

# 2. Why do we need Limine?

Without Limine, your kernel has no idea:

* where the framebuffer is,
* what resolution is active,
* how many bytes each row occupies,
* how pixels are encoded.

So the bootloader provides this information.

```
Kernel
   │
Framebuffer Request
   │
   ▼
Limine
   │
Creates graphics mode
   │
Finds framebuffer
   │
Writes response
   │
Jumps to _start()
```

After `_start()`, Limine is finished.

---

# 3. What information does Limine provide?

Conceptually:

```rust
framebuffer.address()
framebuffer.width()
framebuffer.height()
framebuffer.pitch()
framebuffer.bpp()
```

Example:

```
Address : 0xFFFF800010000000

Width   : 1920
Height  : 1080

Pitch   : 8192 bytes

BPP      : 32 bits
```

---

# 4. Address

Example:

```
Address

0xFFFF800010000000
```

This is **not** pixel zero.

It is the **start of framebuffer memory**.

---

# 5. Width

```
Width = 1920
```

Meaning

```
x

0................1919
```

Valid x coordinates:

```
0

↓

1919
```

---

# 6. Height

```
Height = 1080
```

Meaning

```
y

0
1
2
...
1079
```

---

# 7. BPP (Bits Per Pixel)

Usually

```
32 bits
```

which means

```
4 bytes
```

per pixel.

Example pixel

```
0x00FF0000
```

might represent

```
Red
```

Another

```
0x0000FF00
```

might be

```
Green
```

Another

```
0x000000FF
```

might be

```
Blue
```

(Exact channel order depends on the framebuffer format.)

---

# 8. Why 32 bits?

Because

```
8 bits Red
8 bits Green
8 bits Blue
8 bits Alpha / Reserved
```

```
AAAAAAAA
RRRRRRRR
GGGGGGGG
BBBBBBBB
```

32 bits total.

---

# 9. What is Pitch?

This is probably the most important concept.

Many beginners assume

```
pitch = width × bytes_per_pixel
```

Not always.

Example

```
1920 pixels

×

4 bytes

=

7680 bytes
```

But GPU gives

```
Pitch

8192 bytes
```

Why?

Alignment.

Rows are padded for hardware efficiency.

Memory becomes

```
Row 0

Pixels............
Padding

Row 1

Pixels............
Padding
```

Therefore

```
pitch

≠

width × bpp
```

Never assume.

Always use `framebuffer.pitch()`.

---

# 10. Pixel Address Formula

This is the formula you'll use throughout the kernel.

```
pixel_address

=

base_address

+

y × pitch

+

x × bytes_per_pixel
```

This is the heart of every software renderer.

---

# 11. Why cast to u8?

Suppose

```
base = *mut u32
```

Then

```rust
base.add(1)
```

moves

```
4 bytes
```

because Rust scales pointer arithmetic by the pointed-to type.

For byte arithmetic we need

```rust
*mut u8
```

Then

```rust
add(1)
```

really means

```
1 byte
```

That's why our code does:

```rust
framebuffer
    .addr()
    .cast::<u8>()
```

---

# 12. Why cast back to u32?

Eventually we want to write

```
32-bit pixel
```

So

```rust
.cast::<u32>()
```

lets us do

```rust
write_volatile(color)
```

---

# 13. Why write_volatile()?

Normally the compiler is free to optimize:

```rust
*ptr = value;
```

away if it thinks the value is never read.

But framebuffer memory is **memory-mapped I/O**.

Writing changes hardware state.

So we use

```rust
write_volatile()
```

meaning

> Never optimize this away.

---

# 14. Our put_pixel()

```
put_pixel()

↓

Compute address

↓

Write pixel
```

Everything else builds on this.

---

# 15. draw_box()

```
for every row

    for every column

        put_pixel()
```

So

```
draw_box()

↓

put_pixel()

↓

framebuffer
```

---

# 16. Future Graphics Stack

```
put_pixel()

↓

draw_line()

↓

draw_rectangle()

↓

draw_circle()

↓

draw_bitmap()

↓

draw_character()

↓

draw_string()

↓

terminal

↓

window manager
```

Every graphics library ultimately reaches `put_pixel()`.

---

# 17. Memory Usage

Example:

```
1920 × 1080

×

4 bytes

=

8,294,400 bytes
```

≈

```
7.9 MiB
```

Just one framebuffer.

Double buffering?

```
≈16 MiB
```

Triple buffering?

```
≈24 MiB
```

Modern GPUs allocate far more than this, but it's useful to appreciate the scale.

---

# 18. Coordinate System

```
(0,0)

+---------------------------->

|

|

|

v

y
```

Top-left is the origin.

Increasing:

```
x → right

y ↓ down
```

Unlike mathematical graphs, screen coordinates grow downward.

---

# 19. Color Examples

```
0x00000000
```

Black

```
0x00FFFFFF
```

White

```
0x00FF0000
```

Red

```
0x0000FF00
```

Green

```
0x000000FF
```

Blue

```
0x00FFFF00
```

Yellow

```
0x0000FFFF
```

Cyan

```
0x00FF00FF
```

Magenta

(The exact interpretation depends on the framebuffer's pixel format.)

---

# 20. The biggest takeaway

The framebuffer is **not a graphics API**.

It's just memory.

The kernel's responsibility is to decide:

* what each pixel should be,
* where it belongs,
* and when to update it.

Everything you see—text, icons, windows, cursors, animations—is ultimately produced by writing values into this memory.

---

## Summary

At this point, Nucleus has achieved:

* ✅ Boots through Limine.
* ✅ Receives a framebuffer from the bootloader.
* ✅ Understands framebuffer metadata (address, width, height, pitch, bpp).
* ✅ Can calculate pixel addresses correctly.
* ✅ Can safely write pixels using `write_volatile()`.
* ✅ Has its first graphics primitive: `put_pixel()`.
* ✅ Has its first drawing routine: `draw_box()`.

This is a significant milestone: Nucleus can now produce graphics entirely through its own code. The next subsystem—**the Physical Memory Manager (PMM)**—will give the kernel the ability to manage RAM itself instead of relying solely on what the bootloader prepared.
