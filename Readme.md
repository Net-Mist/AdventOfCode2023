# Advent of rust 2023

My solutions to the Advent of code 2023, in Rust.

I tried to have the fastest solution possible by implementing concepts from [The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html), but also keeping clear Rust code.

## Things learn

- When size of vector are known, use ArrayVec

## generate all days

```sh
for i in {4..25}; do
  cp template/day_N.rs src/day_$i.rs
done
```

```sh
for i in {4..25}; do
  touch inputs/day_$i.txt
done
```
