# Standardizer Dependencies

`standardizer-dependencies` synchronizes machine-managed binary dependencies
used by Standardizer projects.

It discovers the current Rust project, reads the exact
`standardizer-common` version from `Cargo.lock`, resolves the latest compatible
binary-data publication, and installs the required binary files locally.

## Installation

    cargo install standardizer-dependencies --locked

## Usage

Run from the consumer repository:

    standardizer-dependencies

The current Standardizer dependency installs:

- `data/routing.bin`
- `data/tools.bin`

These files are machine-managed and must not be edited manually.

## Development

Install the repository Git hooks with:

    ./.githooks/install

## License

Licensed under the Apache License, Version 2.0. See `LICENSE`.
