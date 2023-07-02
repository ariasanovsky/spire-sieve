# Spire sieve

## Disclaimer

`Slay the Spire` is a registered trademark by `Megacrit, LLC`.
Please support the developers of this excellent game by [purchasing it](https://store.steampowered.com/app/646570/Slay_the_Spire/)!

## Contents

`spire-sieve` is a fan-made Rust crate that provides tools to sieve for `Slay the Spire` game seeds with desired properties.

`spire-sieve` is planned for `no_std`-compatibility for `CUDA` and other environments where the Rust standard library is not available.

## Filter roadmap

| Feature          | backend    | `SeedFilter`   | `no_std`      | no `panic` in asm/ptx |`kani` proof |
|------------------|------------|----------------|---------------|-----------------------|-------------|
| bottleneck map   | 🌱🔬      | 🌱🔬          | 🚧           |                       |             |
| elite bottleneck | 🌱🔬      | 🌱🔬          | 🚧           |                       |             |
| one-path map     | 🌱🔬      | 🌱🔬          | 🚧           |                       |             |
| speedrun map     | 🌱🔬      | 🛠️            |               |                       |             |
| Snecko rolls     | 🛠️        |                |               |                       |             |
| Pandora's Box    | 🌱        | 🌱             | 🚧           |                       |             |
| relic shuffles   | 🚧        |                |               |                       |             |
| card rewards     | 🌱🔬      | 🌱            | 🚧            |                       |             |
| Neow bonuses     | 🌱        | 🛠️             |               |                       |             |
| shop cards       | 🚧        |                |               |                       |             |
| shop relics      | 🚧        |                |               |                       |             |

## Additional features roadmap

| Feature                | description                                | backend  | implementation |
|------------------------|--------------------------------------------|----------|----------------|
| overflow toggle        | toggle RNG overflow guard                  | 🌱      | 🛠️             |
| parallel CPU search    | `Search` with rayon enabled                | 🌱      | 🛠️             |
| parallel GPU search    | `Search` which emits CUDA kernels          | 🌱      | 🚧             |
| filter `PhantomData`   | add optionally printable output to data    | 🚧      |                |
| unlock levels          | adjust filters with save file `Unlocks`    | 🛠️      |                |
| hash skip              | option to filter over `Seed0` over `Seed`  | 🛠️      |                |
| one-path heuristics    | reject prematurely based on paths 1 & 2    | 🛠️      |                |
| `JSON` search settings | `SearchSettings` to specify search params  | 🚧      |                |
| CLI search             | specify search from command line interface | 🚧      |                |
| GUI search             | specify search from graphical interface    | 🚧      |                |
| modded characters      | include `Downfall` content                 | 🚧      |                |

## Contributing

Contributions are welcome!
If you find a bug or have a feature request, please open an issue on the GitHub repository.
If you would like to contribute code, please open a pull request.
