Exactly! 🎉 And you did the math mentally, which is what matters.

Let's verify it:

Pages:

```text
16384
```

Need one bit per page:

```text
16384 bits
```

Convert to bytes:

```text
16384 / 8

=

2048 bytes
```

Only **2 KiB**!

---

Now compare that with a naive approach.

Suppose we stored one `bool` per page.

```rust
bool free[16384];
```

In Rust/C, a `bool` occupies **1 byte**.

So we'd need:

```text
16384 bytes

=

16 KiB
```

Compare:

```text
Bitmap      : 2 KiB
Bool array  : 16 KiB
```

The bitmap is **8× smaller**.

That's why nearly every OS uses bitmaps somewhere.

---

# Now let's understand what a bitmap actually is

Imagine we have only **8 pages**.

Initially every page is free.

We could represent that as:

```text
Page

0 1 2 3 4 5 6 7

Bitmap

00000000
```

Let's define:

```text
0 = Free
1 = Used
```

Now suppose the kernel allocates page 2.

Bitmap becomes

```text
00100000
```

Allocate page 5.

```text
00100100
```

Free page 2.

```text
00000100
```

Nothing magical happened.

We simply changed one bit.

---

# Why bits?

Because CPUs are incredibly good at manipulating bits.

To allocate:

```text
Find first 0

↓

Set it to 1

↓

Return page address
```

To free:

```text
Find corresponding bit

↓

Clear it to 0
```

This is only a few CPU instructions.

---

# Here's something I want you to notice

We never store addresses inside the bitmap.

The bitmap is only:

```text
00100100
```

Where is page 5's address?

We calculate it.

Suppose:

```text
Base = 0x100000
```

Then

```text
Page 0

0x100000

Page 1

0x101000

Page 2

0x102000

Page 3

0x103000
```

Do you see the pattern?

The address is simply:

```text
address = base + page_index × 4096
```

The bitmap only tells us **whether page 5 is free**.

It doesn't store:

```text
0x105000
```

because we can compute it.

That's another huge memory saving.

---

## We're finally ready to write the allocator.

The very first version of `alloc_frame()` will literally do this:

```text
for every bit in bitmap

    if bit == 0

        set bit to 1

        return base + index × 4096
```

That's it.

No fancy algorithms.

No optimization.

Just a linear scan.

And here's the nice part: **it will actually work**.

---

## Next step

In the next session we'll stop talking about concepts and implement:

* A bitmap in memory.
* `set_bit(index)`.
* `clear_bit(index)`.
* `is_bit_set(index)`.
* The first working `alloc_frame()`.

From that point onward, Nucleus will be able to hand out physical pages on its own—the first true resource manager in your kernel.
