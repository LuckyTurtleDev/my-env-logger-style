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

///enabale or disabel showing the module path
pub fn show_module(show: bool) {
	SHOW_MODULE.store(show, Ordering::Relaxed);
}

///enabale or disabel showing emojis before the log level
pub fn show_emoji(show: bool) {
	SHOW_EMOJIS.store(show, Ordering::Relaxed);
}

/// return the current module len.
///
/// And set the module length to the maximum of the current value and the given `len`
pub fn get_set_max_module_len(len: usize) -> usize {
	let module_len = MAX_MODULE_LEN.load(Ordering::Relaxed);
	if module_len < len {
		MAX_MODULE_LEN.store(len, Ordering::Relaxed);
	}
	module_len
}

/// set thi timestamp precision or disable timestamps complete
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
		write!(buf, "{:module_len$} > ", dimmed.value(module))?;
	}

	writeln!(buf, "{}", record.args())
}
