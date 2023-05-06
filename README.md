# my-env-logger-style ![License: none](https://img.shields.io/badge/license-none-blue) [![my-env-logger-style on crates.io](https://img.shields.io/crates/v/my-env-logger-style)](https://crates.io/crates/my-env-logger-style) [![my-env-logger-style on docs.rs](https://docs.rs/my-env-logger-style/badge.svg)](https://docs.rs/my-env-logger-style) [![Source Code Repository](https://img.shields.io/badge/Code-On%20none-blue)](none) ![Rust Version: none](https://img.shields.io/badge/rustc--orange.svg)

A pretty, opinionated style for [env_logger][__link0] inspirated by [pretty-env-logger][__link1].

It is not a goal of this crate to create a feature rich wrapper around [env_logger][__link2]. Instead it does provide a formater, which can be applied to the [`env_logger::Builder`][__link3]. Additional an optional [function][__link4] to create and register a zero config logger is provided.


## Usage


##### Quickstart


```rust
my_env_logger_style::just_log();
info!("Hello, world!");
```

This creates the default env_logger from environment variables and register it as logger.


##### Advance

You can also create an [`env_logger::Builder`][__link5] and apply the style definded at this crate, by using the [`format()`][__link6] function.


```rust
use log::info;
use my_env_logger_style::format;

fn main() {
	env_logger::Builder::new()
		.parse_default_env()
		.format(format)
		.init();
	info!("Hello, world!");
}
```


## Feature-flags


##### time (default)

RFC3339 timestamps


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGyDwipHVMb5RGxgd3zutc1TvG3ARKV4UcQ1NGyM1aXabIPYbYXKEG63oKbYcqLk8Gy1WS8T5S9iBG8yu1HMKwPlTG1nu0eC2mhZ0YWSCgmplbnZfbG9nZ2VyZjAuMTAuMINzbXktZW52LWxvZ2dlci1zdHlsZWUwLjEuMHNteV9lbnZfbG9nZ2VyX3N0eWxl
 [__link0]: https://crates.io/crates/env_logger
 [__link1]: https://crates.io/crates/pretty_env_logger
 [__link2]: https://crates.io/crates/env_logger
 [__link3]: https://docs.rs/env_logger/0.10.0/env_logger/?search=Builder
 [__link4]: https://docs.rs/my-env-logger-style/0.1.0/my_env_logger_style/?search=just_log
 [__link5]: https://docs.rs/env_logger/0.10.0/env_logger/?search=Builder
 [__link6]: https://docs.rs/my-env-logger-style/0.1.0/my_env_logger_style/?search=format
