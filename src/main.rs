#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

use log::*;
use my_env_logger_style::format;

fn main() {
	env_logger::Builder::new()
		.parse_default_env()
		.filter_level(log::LevelFilter::Trace)
		.format(format)
		.format_timestamp_secs()
		.init();
	print_logs();
	foo::print_logs();
	print_logs();
}

fn print_logs() {
	info!("Hello, world!");
	trace!("traceeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
	debug!("bugs everywhere");
	warn!("This looks strange");
	error!("Something went wrong")
}

mod foo {
	use super::*;
	pub(crate) fn print_logs() {
		info!("Hello, from mod");
	}
}
