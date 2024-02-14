# Advent of rust 2023

<!--toc:start-->

- [Advent of rust 2023](#advent-of-rust-2023)
  - [Things learn](#things-learn)
  - [Profiling](#profiling)
  <!--toc:end-->

My solutions to the Advent of code 2023, in Rust.

I tried to have the fastest solution possible by implementing concepts from [The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html), but also keeping clear Rust code. All solutions use a single thread.

On a i7-1165G7, the time spent for each part are:

Day 1 (203.19µs)

- Generator (49.00ns)
- Part 1 (44.31µs) ............... 55172
- Part 2 (158.83µs) .............. 54925

Day 2 (86.19µs)

- Generator (80.91µs)
- Part 1 (2.66µs) ................ 1853
- Part 2 (2.62µs) ................ 72706

Day 3 (94.93µs)

- Generator (11.32µs)
- Part 1 (59.89µs) ............... 507214
- Part 2 (23.71µs) ............... 72553319

Day 4 (213.81µs)

- Generator (5.43µs)
- Part 1 (110.42µs) .............. 25231
- Part 2 (97.96µs) ............... 9721255

Day 5 (52.03µs)

- Generator (35.50µs)
- Part 1 (2.64µs) ................ 836040384
- Part 2 (13.88µs) ............... 10834440

Day 6 (4.24µs)

- Generator (103.00ns)
- Part 1 (2.43µs) ................ 771628
- Part 2 (1.71µs) ................ 27363861

Day 7 (618.44µs)

- Generator (931.00ns)
- Part 1 (287.97µs) .............. 250453939
- Part 2 (329.54µs) .............. 248652697

Day 8 (592.73µs)

- Generator (93.25µs)
- Part 1 (39.49µs) ............... 20513
- Part 2 (460.00µs) .............. 15995167053923

Day 9 (130.46µs)

- Generator (75.82µs)
- Part 1 (27.64µs) ............... 1995001648
- Part 2 (27.00µs) ............... 988

Day 10 (701.60µs)

- Generator (153.92µs)
- Part 1 (93.08µs) ............... 6897
- Part 2 (454.59µs) .............. 367

Day 11 (100.80µs)

- Generator (23.51µs)
- Part 1 (43.49µs) ............... 9684228
- Part 2 (33.79µs) ............... 483844716556

Day 12 (3.66ms)

- Generator (182.32µs)
- Part 1 (308.32µs) .............. 7260
- Part 2 (3.17ms) ................ 1909291258644

Day 13 (95.23µs)

- Generator (30.11µs)
- Part 1 (16.72µs) ............... 33122
- Part 2 (48.40µs) ............... 32312

Day 14 (12.19ms)

- Generator (10.64µs)
- Part 1 (27.94µs) ............... 106517
- Part 2 (12.15ms) ............... 79723

Day 15 (261.34µs)

- Generator (74.80µs)
- Part 1 (45.20µs) ............... 511215
- Part 2 (141.34µs) .............. 236057

Day 16 (10.00ms)

- Generator (12.83µs)
- Part 1 (47.11µs) ............... 8551
- Part 2 (9.94ms) ................ 8754

Day 17 (82.96ms)

- Generator (8.56µs)
- Part 1 (45.42ms) ............... 817
- Part 2 (37.53ms) ............... 925

Day 18 (22.38µs)

- Generator (31.00ns)
- Part 1 (11.83µs) ............... 58550
- Part 2 (10.52µs) ............... 47452118468566

Day 19 (221.03µs)

- Generator (153.58µs)
- Part 1 (30.60µs) ............... 418498
- Part 2 (36.85µs) ............... 123331556462603

Day 20 (1.96ms)

- Generator (19.53µs)
- Part 1 (408.36µs) .............. 919383692
- Part 2 (1.53ms) ................ 247702167614647

Day 21 (3.19ms)

- Generator (39.53µs)
- Part 1 (65.21µs) ............... 3699
- Part 2 (3.09ms) ................ 613391294577878

Day 22 (4.23ms)

- Generator (1.51ms)
- Part 1 (5.91µs) ................ 492
- Part 2 (2.72ms) ................ 86556

Day 23 (185.51ms)

- Generator (146.31µs)
- Part 1 (562.00ns) .............. 2394
- Part 2 (185.36ms) .............. 6554

Day 24 (644.12µs)

- Generator (103.69µs)
- Part 1 (501.26µs) .............. 21679
- Part 2 (39.17µs) ............... 566914635762564

Day 25 (249.96ms)

- Generator (153.17µs)
- Part 1 (249.81ms) .............. 613870
- Part 2 (29.00ns) ............... 1

Overall runtime (558.05ms)

## Things learn

- When size of vector are known, use ArrayVec to prevent Heap allocation.
- At the scale of AoC problems, hashmaps and hashsets are slow. Even using ahash or noHash. Prefer using bitwise operations and vect if possible.
- Creating nested vect, like `vec![vec![false; W]; H]` can be slow because of memory allocations. Prefer `vec![false; W * H]` or bitwise version like `vec![0u64; (W * H + 63)/64]` (`+63` is for rounding up).
- `cargo-show-asm` is great to investigate ASM code. A function can be marked as `#[inline(never)]` if it doesn't appear in the assembly.
- [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) are great tools when computing area of big polygons.

## Profiling

On linux, `perf` is a great tool. It can be run with:

```sh
sudo perf record --call-graph dwarf -F max target/release/aoc -d 21
```

Then `hotspot` can be used to visualize the results.

For comparing solutions, microbenchmarks have been setup. You can run them with `cargo bench`.
