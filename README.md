# Composer RS

An experimental (and WIP) re-imagining of [Composer](https://github.com/composer/composer) written in Rust.

## (Intended) Features

- Near-full compatibility with Composer
  - One major exception is the plugin API, unless I can find a way to run PHP code within Rust
- Built-in [normalization](https://github.com/ergebnis/composer-normalize) support
- Multiple installation strategies
  - Extracting directly to the `vendor/` folder (default)
  - Extracting to a store and symlinking to `vendor/` (like pnpm)
