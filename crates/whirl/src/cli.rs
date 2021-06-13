// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

use structopt::clap::{App, AppSettings, Arg, SubCommand};
use whirl_config::Config;

enum RunType {
  Distributor,
  Hub,
  Api,
}
impl FromStr for RunType {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "distributor" => Ok(Self::Distributor),
      "hub" => Ok(Self::Hub),
      "api" => Ok(Self::Api),
      _ => Err("no match"),
    }
  }
}

// https://stackoverflow.com/questions/57888454/how-to-remove-duplicates-from-a-vector-of-structures
trait Dedup<T: PartialEq + Clone> {
  fn clear_duplicates(&mut self);
}
impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
  fn clear_duplicates(&mut self) {
    let mut already_seen = vec![];
    self.retain(|item| match already_seen.contains(item) {
      true => false,
      _ => {
        already_seen.push(item.clone());
        true
      }
    })
  }
}

pub struct Cli;
impl Cli {
  /// # Panics
  /// Though Rust thinks a panic might happen because of the `unreachable`
  /// macro, all CLI subcommands are handled so it is trully unreachable.
  pub async fn execute() {
    let matches = Self::cli().get_matches();

    if Config::get().whirlsplash.log.test {
      error!("error");
      warn!("warn");
      info!("info");
      debug!("debug");
      trace!("trace");
    }

    debug!("attempting to create .whirl directory...");
    match std::fs::create_dir(".whirl/") {
      Ok(_) => debug!("successfully created .whirl directory"),
      Err(e) => debug!("error creating .whirl directory: {}", e),
    }

    match matches.subcommand() {
      ("run", Some(s_matches)) =>
        Self::run({
          // Make a vector of the passed types for the `run` sub-command, if the
          // vector cannot be unwrapped, the must mean that the user did not
          // pass any sub-server types meaning that the subcommand `run` was
          // called without any arguments, in other words: "run everything".
          let mut types = {
            // s_matches.values_of("type").unwrap().collect::<Vec<_>>();
            match s_matches.values_of("type") {
              Some(values) => values.collect::<Vec<_>>(),
              None => vec!["distributor", "hub", "api"],
            }
          };
          // Remove any duplicate sub-commands, we don't want to start two
          // instances of the same sub-server.
          types.clear_duplicates();

          let mut run_types = vec![];
          // Iterate over all of the types and push them to a vector that we'll
          // pass to the `run` method.
          loop {
            match types.last() {
              Some(run_type) => match run_type.to_owned() {
                "distributor" => run_types.push(RunType::Distributor),
                "hub" => run_types.push(RunType::Hub),
                "api" => run_types.push(RunType::Api),
                _ => {}
              },
              None => break,
            }
            types.pop();
          }

          run_types
        })
        .await,
      ("config", Some(s_matches)) =>
        match s_matches.subcommand() {
          ("show", _) => println!("{:#?}", Config::get()),
          _ => unreachable!(),
        },
      ("clean", _) => {
        let cleanable_directories = vec![".whirl/log/"];
        for dir in cleanable_directories {
          let file_type = if dir.ends_with('/') {
            "directory"
          } else {
            "file"
          };
          info!("cleaning {}: {}", file_type, dir);
          if let Err(e) = std::fs::remove_dir_all(dir) {
            warn!("cannot delete {}: {}: {}", file_type, dir, e);
          }
        }
      }
      _ => unreachable!(),
    }
  }

  fn cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .settings(&[AppSettings::SubcommandRequiredElseHelp])
      .subcommands(vec![
        SubCommand::with_name("run")
          .about("Start the WorldServer or a selection of sub-servers.")
          .long_about("Start the WorldServer by running this sub-command \
          WITHOUT any arguments, start a selection of sub-servers by passing a \
          comma-separated list of sub-server types.")
          .arg(
            Arg::with_name("type")
              .required(false)
              .takes_value(true)
              .index(1)
              .use_delimiter(true)
              .possible_values(&["distributor", "hub", "api"]),
          ),
        SubCommand::with_name("config")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .subcommands(vec![SubCommand::with_name("show")]),
        SubCommand::with_name("clean").about(
          "Delete Whirl-generated files/ directories which are NOT critical. E.g., .whirl/logs/",
        ),
      ])
      .args(&[
        Arg::with_name("debug").short("d").long("debug"),
        Arg::with_name("trace").short("t").long("trace"),
      ])
  }

  async fn run(mut server_type: Vec<RunType>) {
    let mut threads = vec![];
    // Iterate over all of of the requested sub-servers and spawn one of each.
    loop {
      match server_type.last() {
        Some(run_type) => match run_type {
          RunType::Distributor => threads.push(whirl_server::make::distributor()),
          RunType::Hub => threads.push(whirl_server::make::hub()),
          RunType::Api => threads.push(whirl_api::make()),
        },
        None => break,
      }
      server_type.pop();
    }

    if std::env::var("DISABLE_PROMPT").unwrap_or_else(|_| "false".to_string()) == "true"
      || !Config::get().whirlsplash.prompt.enable
    {
      info!("starting with prompt disabled");
      loop {
        std::thread::sleep(std::time::Duration::default());
      }
    }

    whirl_prompt::Prompt::handle().await;
  }
}
