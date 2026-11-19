# Source reference

This project follows Philipp Oppermann's [Writing an OS in Rust](https://os.phil-opp.com/)
through [Heap Allocation](https://os.phil-opp.com/heap-allocation/).

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
This stops at post 10; Allocator Designs and later chapters are not included.

Build a bootable image with `cargo bootimage`, run it with `cargo run`, and run the
upstream unit and integration tests in QEMU with `cargo test`.
