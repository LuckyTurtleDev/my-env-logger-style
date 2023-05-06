# my-env-logger-style ![License: none](https://img.shields.io/badge/license-none-blue) [![my-env-logger-style on crates.io](https://img.shields.io/crates/v/my-env-logger-style)](https://crates.io/crates/my-env-logger-style) [![my-env-logger-style on docs.rs](https://docs.rs/my-env-logger-style/badge.svg)](https://docs.rs/my-env-logger-style) [![Source Code Repository](https://img.shields.io/badge/Code-On%20none-blue)](none) ![Rust Version: none](https://img.shields.io/badge/rustc--orange.svg)

A pretty, opinionated style for [env_logger][__link0] inspirated by [pretty-env-logger][__link1].

It is not a goal of this crate to create a feature rich wrapper around [env_logger][__link2]. Instead it does provide a formater, which can be applied to the [`env_logger::Builder`][__link3]. Additional an optional [function][__link4] to create and register a zero config logger is provided.

Timestamp, emojis and modules can be disable separately.


## Preview

![image][__link5]

with timestamps:

![image][__link6]


## Usage


##### Quickstart


```rust
my_env_logger_style::just_log();
info!("Hello, world!");
```

This creates the default env_logger from environment variables and register it as logger.


##### Advance

You can also create an [`env_logger::Builder`][__link7] and apply the style definded at this crate, by using the [`format()`][__link8] function.


```rust
use log::info;
use my_env_logger_style::format;

env_logger::Builder::new()
	.parse_default_env()
	.format(format)
	.init();
info!("Hello, world!");
```


## Feature-flags


##### time (default)

RFC3339 timestamps


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGyDwipHVMb5RGxgd3zutc1TvG3ARKV4UcQ1NGyM1aXabIPYbYXKEG7EFv_7UCGLAG6p_0yQ4EO5pG2oTs9qOJvabG93PJiAv3uw6YWSCgmplbnZfbG9nZ2VyZjAuMTAuMINzbXktZW52LWxvZ2dlci1zdHlsZWUwLjEuMHNteV9lbnZfbG9nZ2VyX3N0eWxl
 [__link0]: https://crates.io/crates/env_logger
 [__link1]: https://crates.io/crates/pretty_env_logger
 [__link2]: https://crates.io/crates/env_logger
 [__link3]: https://docs.rs/env_logger/0.10.0/env_logger/?search=Builder
 [__link4]: https://docs.rs/my-env-logger-style/0.1.0/my_env_logger_style/?search=just_log
 [__link5]: https://user-images.githubusercontent.com/44570204/236641121-5071e42a-9f9b-4bff-a6fb-03ff294f5d9e.png
 [__link6]: https://user-images.githubusercontent.com/44570204/236641172-fb304d1f-7e50-4283-969e-949a76b0ba00.png
 [__link7]: https://docs.rs/env_logger/0.10.0/env_logger/?search=Builder
 [__link8]: https://docs.rs/my-env-logger-style/0.1.0/my_env_logger_style/?search=format
