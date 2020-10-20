# lfg

![CI](https://github.com/Celeo/lfg/workflows/CI/badge.svg?branch=master)

Implementation of an LFG system from many videogames. Why? Because I thought it'd be interesting. Note that this isn't any sort of interface to any game's LFG system.

## Project goals

I'll basically be basing this project off of FFXIV's Duty Finder, if you're curious.

This system should be able to:

- Define a number of dungeons
  - Each dungeon will require a certain number of each role (tank, healer, striker)
- Characters can queue for between 1 and 5 dungeons at a time
- The system attempts to make groups of players from the queue
- Player's position in the queue is used to best-fit potential groups, but won't block other groups from being formed

Future:

- Duty join confirmation
- Join in-duty

## Installing

1. Clone the repo and build the binary

## Developing

### Building

### Requirements

- Git
- A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/lfg
cd lfg
cargo build
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
