use env_logger::fmt::Formatter;
use log::{Level, Record};
use std::{
	io,
	io::Write,
	sync::atomic::{AtomicBool, AtomicUsize, Ordering}
};

static MAX_MODULE_LEN: AtomicUsize = AtomicUsize::new(0);
static SHOW_MODULE: AtomicBool = AtomicBool::new(true);
static SHOW_EMOJIS: AtomicBool = AtomicBool::new(true);

///create and regstier a logger from the default environment variables
pub fn just_log() {
	env_logger::Builder::new()
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

///log formater witch can be used at the [`format()`](env_logger::Builder::format()) function of the [`env_logger::Builder`].
pub fn format(buf: &mut Formatter, record: &Record) -> io::Result<()> {
	let mut bold = buf.style();
	bold.set_bold(true);

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

	let mut module_style = buf.style();
	module_style.set_dimmed(true); //grey
	let module = if SHOW_MODULE.load(Ordering::Relaxed) {
		record.module_path().unwrap_or_default()
	} else {
		""
	};
	let module_len = MAX_MODULE_LEN.load(Ordering::Relaxed);
	if module_len < module.len() {
		MAX_MODULE_LEN.store(module.len(), Ordering::Relaxed);
	}
	let mod_separator = if !module.is_empty() { " > " } else { "" };

	writeln!(
		buf,
		"{level_symbol} {:6} {:module_len$}{}{}",
		level_style.value(record.level()),
		module_style.value(module),
		bold.value(mod_separator),
		record.args()
	)
}
