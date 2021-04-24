use structopt::clap::{App, AppSettings, Arg, SubCommand};

pub fn cli<'b, 'a>() -> App<'a, 'b> {
  App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .settings(&[AppSettings::SubcommandRequiredElseHelp])
    .subcommands(vec![
      SubCommand::with_name("run").about("Start the WorldServer"),
      SubCommand::with_name("config")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![SubCommand::with_name("show")]),
      SubCommand::with_name("completions")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Generate shell completions")
        .subcommands(vec![
          SubCommand::with_name("powershell"),
          SubCommand::with_name("bash"),
          SubCommand::with_name("elvish")
          SubCommand::with_name("zsh"),
          SubCommand::with_name("fish"),
        ]),
    ])
    .args(&[
      Arg::with_name("debug").short("d").long("debug"),
      Arg::with_name("trace").short("t").long("trace"),
    ])
}
