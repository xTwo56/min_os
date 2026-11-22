# Source reference

This project contains a bare-metal Rust kernel with paging, heap allocation,
custom allocators, and cooperative asynchronous tasks.

## Asynchronous task execution

- `src/task/mod.rs` owns pinned futures and assigns unique task identifiers.
- `src/task/executor.rs` schedules only ready tasks through a bounded queue and
  custom wakers, then halts the CPU while no work is ready.
- `src/task/keyboard.rs` transfers keyboard bytes from interrupt context into an
  asynchronous stream for decoding and printing.
- `src/task/simple_executor.rs` retains the minimal polling executor as a compact
  reference implementation.
- `src/interrupts.rs` now limits the keyboard handler to reading and enqueueing a
  scancode, while `src/main.rs` starts the executor and its tasks.

## Allocator designs (post 11)

The allocator implementations were copied from the official
[`phil-opp/blog_os` `post-11` branch](https://github.com/phil-opp/blog_os/tree/8a2d25798dfb12cbd11bc38df678beda9ad5a676),
commit `8a2d25798dfb12cbd11bc38df678beda9ad5a676`:

- `src/allocator/bump.rs` implements the article's linear bump allocator.
- `src/allocator/linked_list.rs` implements the article's first-fit free-list
  allocator with metadata stored in free memory.
- `src/allocator/fixed_size_block.rs` implements the selected global allocator:
  fixed-size free lists for common small layouts and `linked_list_allocator::Heap`
  as the fallback for new blocks and larger layouts.
- `src/allocator.rs` adds the `Locked` wrapper and `align_up` helper, and switches
  the global allocator from `LockedHeap` to `FixedSizeBlockAllocator`.
- `tests/heap_allocation.rs` adds the upstream long-lived allocation test.

The algorithms match upstream. Local changes are explanatory comments, formatting,
and the existing `min_os` crate name.

## Heap allocation (post 10)

The heap changes were copied from the official
[`phil-opp/blog_os` `post-10` branch](https://github.com/phil-opp/blog_os/tree/ab76226d5510a10b5e7736a3249a28368b260369),
commit `ab76226d5510a10b5e7736a3249a28368b260369`:

- `src/allocator.rs` maps a 100 KiB heap at virtual address `0x444444440000`
  and initializes the global `LockedHeap`. Upstream's educational `Dummy`
  allocator is retained but is not registered as the global allocator.
- `src/main.rs` replaces the VGA mapping example with heap initialization and
  upstream's `Box`, `Vec`, and `Rc` demonstrations.
- `src/memory.rs` removes the old VGA mapping helper, as upstream does.
- `tests/heap_allocation.rs` copies all three upstream heap tests.
- `src/lib.rs` exposes the allocator module and links `alloc`;
  `.cargo/config.toml` adds `alloc` to `build-std` for the custom target.
- `Cargo.toml` adds upstream's `linked_list_allocator = "0.9.0"` requirement.
  Cargo resolves it to compatible version 0.9.1 in `Cargo.lock`, with its
  `spinning_top`, `lock_api`, and `scopeguard` dependencies.

Local adaptations are the `min_os` crate name, the existing greeting, formatting,
and explanatory comments. Existing dependencies and target configuration are
otherwise retained. No allocator algorithm was independently implemented.

## Paging baseline (post 09)

The added implementation was copied from the official
[`phil-opp/blog_os` `post-09` branch](https://github.com/phil-opp/blog_os/tree/3d6647737de4121e26aa64e05a092ff611a990b9),
commit `3d6647737de4121e26aa64e05a092ff611a990b9`:

- `src/memory.rs` originally matched upstream (now advanced to post 10 above).
- `src/main.rs` (since advanced to post 10) and `tests/stack_overflow.rs` use `min_os` instead of `blog_os`;
  the existing greeting is preserved, and imports are formatted for the local name.
- `src/gdt.rs` uses `add_entry` instead of `append` for the existing `x86_64` 0.14 dependency.
- The double-fault handler, IDT registration, breakpoint test, and library test
  entry-point fixes come from the same branch. Existing equivalent code and comments
  are retained elsewhere.
- Existing dependency versions and the `x86_64-min_os.json` target are retained.
  The upstream nightly toolchain selection and stack-overflow test configuration
  are included.

The copied code is used under upstream's MIT license, included in `LICENSE-MIT`.
The kernel now includes asynchronous task execution and keyboard input.

Build a bootable image with `cargo bootimage`, run it with `cargo run`, and run the
upstream unit and integration tests in QEMU with `cargo test`.
