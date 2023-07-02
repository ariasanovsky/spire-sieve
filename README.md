# Spire sieve

`spire-sieve` is a Rust crate that provides tools to sieve for `Slay the Spire` game seeds with desired properties.
It is planned to be a mostly `no_std` for compatibility with `CUDA` or other environments where the Rust standard library is not available.

## Filter feature map

| Feature          | backend    | `SeedFilter`   | `no_std`       | no `panic` in asm/ptx |`kani` proof |
|------------------|------------|----------------|----------------|-----------------------|-------------|
| Bottleneck map   | 🌱         | 🛠️            |                |                       |             |
| Bad bottleneck   | 🌱         | 🛠️            |                |                       |             |
| One-path map     | 🌱         | 🛠️            |                |                       |             |
| Speedrun map     | 🌱         | 🛠️            |                |                       |             |
| Bad Snecko rolls | 🛠️         | 🛠️            |                |                       |             |
| Pandora's box    | 🌱         | 🌱            | 🚧            |                       |             |
| Relic shuffles   | 🛠️         | 🛠️            |                |                       |             |
| Card rewards     | 🌱         | 🌱            | 🚧            |                       |             |
| Neow bonuses     | 🌱         | 🛠️            |                |                       |             |
| Shop cards       | 🚧         | 🛠️            |                |                       |             |
| Shop relics      | 🚧         | 🛠️            |                |                       |             |

## Other features map

| Feature              | description                                | backend  | implementation |
|----------------------|--------------------------------------------|----------|----------------|
| overflow toggle      | toggle RNG overflow guard                  | 🌱      | 🛠️             |
| parallel CPU search  | `Search` with rayon enabled                | 🌱      | 🛠️             |
| parallel GPU search  | `Search` which emits CUDA kernels          | 🌱      | 🛠️             |
| unlock levels        | adjust filters with save file `Unlocks`    | 🛠️      |                |
| hash skip            | option to filter over `Seed0` over `Seed`  | 🛠️      |                |
| one-path heuristics  | reject prematurely based on paths 1 & 2    | 🛠️      |                |
| JSON search settings | `SearchSettings` to specify search params  | 🚧      |                |
| CLI search           | specify search from command line interface | 🚧      |                |
| GUI search           | specify search from graphical interface    | 🚧      |                |
| modded characters    | include `Downfall` content                 | 🚧      |                |

## Contributing

Contributions are welcome!
If you find a bug or have a feature request, please open an issue on the GitHub repository.
If you would like to contribute code, please open a pull request.
