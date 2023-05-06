//! A pretty, opinionated style for [env_logger](https://crates.io/crates/env_logger) inspirated by [pretty-env-logger](https://crates.io/crates/pretty_env_logger).
//!
//! It is not a goal of this crate to create a feature rich wrapper around [env_logger](https://crates.io/crates/env_logger).
//! Instead it does provide a formater, which can be applied to the [`env_logger::Builder`].
//! Additional an optional [function](just_log) to create and register a zero config logger is provided.
//!
//! Timestamp, emojis and modules can be disable separately.
//!
//! # Preview
//! 
//! ![image](https://user-images.githubusercontent.com/44570204/236641121-5071e42a-9f9b-4bff-a6fb-03ff294f5d9e.png)
//! 
//! with timestamps:
//! 
//! ![image](https://user-images.githubusercontent.com/44570204/236641172-fb304d1f-7e50-4283-969e-949a76b0ba00.png)
//! # Usage
//! #### Quickstart
//! ```
//! # use log::info;
//! my_env_logger_style::just_log();
//! info!("Hello, world!");
//! ```
//! This creates the default env_logger from environment variables and register it as logger.
//! #### Advance
//! You can also create an [`env_logger::Builder`] and apply the style definded at this crate,
//! by using the [`format()`] function.
//! ```
//! use log::info;
//! use my_env_logger_style::format;
//!
//! fn main() {
//! 	env_logger::Builder::new()
//! 		.parse_default_env()
//! 		.format(format)
//! 		.init();
//! 	info!("Hello, world!");
//! }
//! ```
//! # Feature-flags
//! #### time (default)
//! RFC3339 timestamps
use env_logger::fmt::Formatter;
use log::{Level, Record};
use std::{
	io,
	io::Write,
	sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering}
};

static MAX_MODULE_LEN: AtomicUsize = AtomicUsize::new(0);
static SHOW_MODULE: AtomicBool = AtomicBool::new(true);
static SHOW_EMOJIS: AtomicBool = AtomicBool::new(true);
static SHOW_TIME: AtomicU8 = AtomicU8::new(TimestampPrecision::Seconds as u8);

pub use env_logger;

#[repr(u8)]
/// RFC3339 timestamps
pub enum TimestampPrecision {
	Disable,
	Seconds,
	Millis,
	Micros,
	Nanos
}

///create and regstier a logger from the default environment variables
pub fn just_log() {
	env_logger::Builder::new()
		.filter_level(log::LevelFilter::Info) //set defaul log level
		.parse_default_env()
		.format(format)
		.init();
}

///enable or disabel showing the module path
pub fn show_module(show: bool) {
	SHOW_MODULE.store(show, Ordering::Relaxed);
}

///enable or disabel showing emojis before the log level
pub fn show_emoji(show: bool) {
	SHOW_EMOJIS.store(show, Ordering::Relaxed);
}

/// return the current module len and set the module length to the maximum of the current value and the given `len`.
///
/// Usefull if you already know the length of module and would like to have an consistant indentation from the beginnig.
pub fn get_set_max_module_len(len: usize) -> usize {
	let module_len = MAX_MODULE_LEN.load(Ordering::Relaxed);
	if module_len < len {
		MAX_MODULE_LEN.store(len, Ordering::Relaxed);
	}
	module_len
}

/// set the timestamp precision or disable timestamps complete
pub fn set_timestamp_precision(timestamp_precission: TimestampPrecision) {
	SHOW_TIME.store(timestamp_precission as u8, Ordering::Relaxed);
}

///log formater witch can be used at the [`format()`](env_logger::Builder::format()) function of the [`env_logger::Builder`].
pub fn format(buf: &mut Formatter, record: &Record) -> io::Result<()> {
	let mut bold = buf.style();
	bold.set_bold(true);
	let mut dimmed = buf.style();
	dimmed.set_dimmed(true);

	{
		let show_time = SHOW_TIME.load(Ordering::Relaxed);
		// safety: SHOW_TIME is inilized with TimestampPrecision::Seconds
		// and can only be written by using set_timestamp_precision()
		match unsafe { std::mem::transmute(show_time) } {
			TimestampPrecision::Disable => Ok(()),
			TimestampPrecision::Seconds => {
				write!(buf, "{} ", dimmed.value(buf.timestamp_seconds()))
			},
			TimestampPrecision::Millis => {
				write!(buf, "{} ", dimmed.value(buf.timestamp_seconds()))
			},
			TimestampPrecision::Micros => {
				write!(buf, "{} ", dimmed.value(buf.timestamp_seconds()))
			},
			TimestampPrecision::Nanos => {
				write!(buf, "{} ", dimmed.value(buf.timestamp_seconds()))
			}
		}?;
	}

	let level_style = buf.default_level_style(record.level());
	let level_symbol = if SHOW_EMOJIS.load(Ordering::Relaxed) {
		match record.level() {
			//💥 and 🔬 are 2 chars big at the terminal. How does it look with other fonts/terminals?
			Level::Trace => "🔬",
			Level::Debug => " ⚙️",
			Level::Info => " ℹ",
			Level::Warn => " ⚠",
			Level::Error => "💥"
		}
	} else {
		""
	};
	write!(
		buf,
		"{level_symbol} {:5} ",
		level_style.value(record.level())
	)?;

	if SHOW_MODULE.load(Ordering::Relaxed) {
		let module = record.module_path().unwrap_or_default();
		let module_len = get_set_max_module_len(module.len());
		write!(
			buf,
			"{:module_len$} {} ",
			dimmed.value(module),
			bold.value('>')
		)?;
	}

	writeln!(buf, "{}", record.args())
}
