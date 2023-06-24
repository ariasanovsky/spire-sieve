## Spire sieve

`spire-sieve` is a Rust crate that provides tools to sieve for `Slay the Spire` game seeds with desired properties.
It is planned to be a mostly `no_std` for compatibility with `CUDA` or other environments where the Rust standard library is not available.

## Planned filters

- **unlucky maps**: e.g., burning elite position & buff, forced burning elite combat on floor 6
- **fast maps**: avoid combats between the first floor and the act boss
- **line maps**: maps with with only 1 node per floor for the first few floors
- **bad Snecko**: runs where Snecko Eye randomizes too many cards to high costs
- **constant Pandora's Box**: runs where Pandora's Box generates many identical cards
- **relic pool shuffles**: for runs which have a desired relic pool shuffle
- **card & potion generation**: for runs which generate specific cards and potions

## Planned features

- **unlock levels**: control the number of unlocks $0$ through $5$ of each character
- **overflow toggling**: control overflow checking during random number generation
- **fast hash**: skip one call to the murmur hash function by starting with `seed0` instead of the run seed
- **filter heuristics**: for constraints which sacrifice correctness for throughput

## Contributing

Contributions are welcome!
If you find a bug or have a feature request, please open an issue on the GitHub repository.
If you would like to contribute code, please open a pull request.
