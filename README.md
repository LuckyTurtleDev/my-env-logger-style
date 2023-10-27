# my-env-logger-style ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![my-env-logger-style on crates.io](https://img.shields.io/crates/v/my-env-logger-style)](https://crates.io/crates/my-env-logger-style) [![my-env-logger-style on docs.rs](https://docs.rs/my-env-logger-style/badge.svg)](https://docs.rs/my-env-logger-style) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/my-env-logger-style)

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

Enable RFC3339 timestamps


##### custom-arg-formatter

Allow using a custom formater to format the args (the actual message) of the log record. As example this can be used to avoid logging private userdata.



 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEG_W_Gn_kaocAGwCcVPfenh7eGy6gYLEwyIe4G6-xw_FwcbpjYXKEG7Vk-MZ5Y-OPG3Gyn8-t-rjgGyewtMniif7nG-E9Audz2fu_YWSCgmplbnZfbG9nZ2VyZjAuMTAuMINzbXktZW52LWxvZ2dlci1zdHlsZWUwLjEuMHNteV9lbnZfbG9nZ2VyX3N0eWxl
 [__link0]: https://crates.io/crates/env_logger
 [__link1]: https://crates.io/crates/pretty_env_logger
 [__link2]: https://crates.io/crates/env_logger
 [__link3]: https://docs.rs/env_logger/0.10.0/env_logger/?search=Builder
 [__link4]: https://docs.rs/my-env-logger-style/0.1.0/my_env_logger_style/?search=just_log
 [__link5]: https://user-images.githubusercontent.com/44570204/236641121-5071e42a-9f9b-4bff-a6fb-03ff294f5d9e.png
 [__link6]: https://user-images.githubusercontent.com/44570204/236641172-fb304d1f-7e50-4283-969e-949a76b0ba00.png
 [__link7]: https://docs.rs/env_logger/0.10.0/env_logger/?search=Builder
 [__link8]: https://docs.rs/my-env-logger-style/0.1.0/my_env_logger_style/?search=format
