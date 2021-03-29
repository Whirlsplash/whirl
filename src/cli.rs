use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
	Run,
	Config,
}

#[derive(StructOpt, Debug)]
#[structopt(
name = env!("CARGO_PKG_NAME"),
about = env!("CARGO_PKG_DESCRIPTION"),
version = env!("CARGO_PKG_VERSION"),
author = env!("CARGO_PKG_AUTHORS"),
)]
pub struct Opt {
	#[structopt(short, long)]
	pub debug: bool,

	#[structopt(short, long, parse(from_occurrences))]
	pub verbose: u8,

	#[structopt(subcommand)] // help
	pub command: Command,
}
