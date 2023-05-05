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
	trace!("traceeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
	debug!("bugs everywhere");
	warn!("This looks strange");
	error!("Something went wrong")
}

mod foo {
	use super::*;
	pub fn print_logs() {
		info!("Hello, from mod");
	}
}
