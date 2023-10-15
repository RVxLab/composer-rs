# Composer RS

An experimental (and WIP) re-imagining of [Composer](https://github.com/composer/composer) written in Rust.

## (Intended) Features

- Near-full API compatibility with Composer
- Built-in [normalization](https://github.com/ergebnis/composer-normalize) support
- Multiple installation strategies
  - Extracting directly to the `vendor/` folder (default)
  - Extracting to a store and symlinking to `vendor/` (like pnpm)

## Implemented features

*TODO*

## Differences to Composer

While I strive to be as compatible with Composer as possible, there are a few differences.

### Config

Several configuration options are either unsupported (such as the PHP-specific ones) and some options have different defaults.

#### Unsupported config and env vars

The following environment variables are specific to PHP and have no effect:

- [`COMPOSER_ALLOW_XDEBUG`](https://getcomposer.org/doc/03-cli.md#composer-allow-xdebug)
- [`COMPOSER_DISABLE_XDEBUG_WARN`](https://getcomposer.org/doc/03-cli.md#composer-disable-xdebug-warn)
- [`COMPOSER_MEMORY_LIMIT`](https://getcomposer.org/doc/03-cli.md#composer-memory-limit)

#### Audits

`audit.abandoned` is set to `"report"` in Composer 2.6 and below, with `"fail"` becoming the default as of version 2.7. Because of that, `"fail"` is considered the default for composer-rs.

#### No suggest

Because the `--no-suggest` flag has been considered deprecated since Composer 2.0 and will be removed in 3.0, this flag has not been implemented.
