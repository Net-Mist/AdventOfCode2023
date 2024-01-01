# Advent of rust 2023

My solutions to the Advent of code 2023, in Rust.

I tried to have the fastest solution possible by implementing concepts from [The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html), but also keeping clear Rust code.

## Things learn

- When size of vector are known, use ArrayVec to prevent Heap allocation.
- at the scale of AoC problems, hashes are slow. Even ahash or noHash. Prefer using bitwise operations if possible.
- Created nested vect, like `vec![vec![false; W]; H]` can be slow because of memory allocations. Prefer `vec![false; W * H]` or bitwise version like `vec![0u64; (W * H + 63)/64]` (`+63` is for rounding up).
- `cargo-show-asm` is great to investigate ASM code. A function can be marked as `#[inline(never)]` if it doesn't appear in the assembly.
- [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) are great tools when computing area of big polygons.

## Profiling

On linux, `perf` is a great tool. It can be run with:

```sh
sudo perf record --call-graph dwarf -F max target/release/aoc -d 21
```

Then `hotspot` can be used to visualize the results.
