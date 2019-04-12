# cargo-dep-sort

Rust command-line utility built to satisfy @dtolnay's [request for implementation #29](https://github.com/dtolnay/request-for-implementation/issues/29).

Checks to ensure that the deps in a given Cargo.toml file are properly ordered so as to minimize issues with merges on projects with multiple contributors.

## Demo

1) Clone this repo, and ensure you have Rust enviornment set up.

2) Test the utility on the unsorted Cargo.toml from the wonderful [rustlings repo](https://github.com/rust-lang/rustlings/).

```
cargo run test_data/rustlings.toml
```

3) Test the utility on the sorted Cargo.toml from this repo.

```
cargo run Cargo.toml
```

