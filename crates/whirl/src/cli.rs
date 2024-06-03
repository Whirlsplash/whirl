// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use {
  std::{io::Write, str::FromStr},
  structopt::clap::{App, AppSettings, Arg, SubCommand},
  whirl_config::Config,
};

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
    self.retain(|item| {
      if already_seen.contains(item) {
        false
      } else {
        already_seen.push(item.clone());

        true
      }
    });
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
      Ok(()) => debug!("successfully created .whirl directory"),
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
            s_matches.values_of("type").map_or_else(
              || vec!["distributor", "hub", "api"],
              std::iter::Iterator::collect,
            )
          };
          // Remove any duplicate sub-commands, we don't want to start two
          // instances of the same sub-server.
          types.clear_duplicates();

          let mut run_types = vec![];
          // Iterate over all of the types and push them to a vector that we'll
          // pass to the `run` method.
          while let Some(run_type) = types.last() {
            match run_type.to_owned() {
              "distributor" => run_types.push(RunType::Distributor),
              "hub" => run_types.push(RunType::Hub),
              "api" => run_types.push(RunType::Api),
              _ => {}
            }
            types.pop();
          }

          run_types
        })
        .await,
      ("config", Some(s_matches)) => match s_matches.subcommand() {
        ("show", _) => println!("{:#?}", Config::get()),
        ("generate", Some(s_s_matches)) => {
          if std::path::Path::new(".whirl/Config.toml").exists()
            && !s_s_matches.is_present("force")
          {
            info!(
              "a configuration file is already present, if you would like to \
               regenerate the configuration file, execute this sub-command \
               with the `--force` (`-f`) flag"
            );
          } else {
            let mut file = std::fs::File::create(".whirl/Config.toml")
              .expect("unable to create configuration file");
            file
              .write_all(include_bytes!(
                "../../whirl_config/Config.default.toml"
              ))
              .expect(
                "unable to write default configuration to generated \
                 configuration file",
              );
            info!("successfully generated a new configuration file");
          }
        }
        _ => unreachable!(),
      },
      ("clean", _) => {
        let cleanable_directories = vec![".whirl/log/"];
        for dir in cleanable_directories {
          let file_type = if dir.ends_with('/') { "directory" } else { "file" };
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
          .long_about(
            "Start the WorldServer by executing this sub-command WITHOUT any \
             arguments, start a selection of sub-servers by passing a \
             comma-separated list of sub-server types.",
          )
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
          .subcommands(vec![
            SubCommand::with_name("show"),
            SubCommand::with_name("generate")
              .alias("gen")
              .arg(Arg::with_name("force").short("f").long("force")),
          ]),
        SubCommand::with_name("clean").about(
          "Delete Whirl-generated files/ directories which are NOT critical. \
           E.g., .whirl/logs/",
        ),
      ])
      .args(&[
        Arg::with_name("debug").short("d").long("debug"),
        Arg::with_name("trace").short("t").long("trace"),
      ])
  }

  async fn run(mut server_type: Vec<RunType>) {
    #[allow(clippy::collection_is_never_read)]
    let mut threads = vec![];
    // Iterate over all of of the requested sub-servers and spawn one of each.
    while let Some(run_type) = server_type.last() {
      match run_type {
        RunType::Distributor => threads.push(whirl_server::make::distributor()),
        RunType::Hub => threads.push(whirl_server::make::hub()),
        RunType::Api => threads.push(whirl_api::make()),
      }
      server_type.pop();
    }

    if std::env::var("DISABLE_PROMPT").unwrap_or_else(|_| "false".to_string())
      == "true"
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
