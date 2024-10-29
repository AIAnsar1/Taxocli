#[allow(non_snake_case)]
pub mod Support;
#[allow(non_snake_case)]
pub mod Api;
#[allow(non_snake_case)]
pub mod Latency;
#[allow(non_snake_case)]
pub mod Config;
#[allow(non_snake_case)]
pub mod Cli;
#[allow(non_snake_case)]
pub mod Reptilia;


use std::{error::Error, time::Duration};
use clap::{Arg, ArgAction, Command};
use crate::Config::Configuration::ExecArgs;
use crate::Cli::Runtime::{IntervalToSeconds, WriteOutputJson, WriteOutputTable, WriteOutputYaml};



fn main() -> Result<(), Box<dyn Error>>{
    #[allow(non_snake_case)]
    let Matches = Command::new("exec").about("Exec The Run Command")
        .arg(Arg::new("Target-URL").short("T").long("Target-URL").help("The Target URL, e.g., https://google.com").required(true).action(ArgAction::Set))
        .arg(Arg::new("Runs").short("R").long("Runs").help("The Number Of Executions.").default_value("1").action(ArgAction::Set))
        .arg(Arg::new("Interval").short("I").long("Interval").help("The amount of waiting time between runs.").default_value("1m").action(ArgAction::Set))
        .arg(Arg::new("Locations").short("L").long("Locations").help("The array of locations to be requested, e.g., NA.US.*,NA.EU.*").default_value("EU.ES.*").action(ArgAction::Set))
        .arg(Arg::new("Output-Locations").short("L").long("Output-Locations").help("The number of best locations to output.").default_value("1m").action(ArgAction::Set))
        .arg(Arg::new("Output-Format").short("L").long("Output-Format").help("Output format: table, yaml, or json").default_value("Table").action(ArgAction::Set))
        .get_matches();
    #[allow(non_snake_case)]
    let Args = ExecArgs {
        TargetUrl: Matches.get_one::<String>("Target-URL").unwrap().to_string(),
        NumberOfRuns: Matches.get_one::<String>("Runs").unwrap().parse().expect("Invalid Number Format"),
        WaitInterval: Matches.get_one::<String>("Interval").unwrap().to_string(),
        Locations: Matches.get_many::<String>("Locations").unwrap().map(|s|s.to_string()).collect(),
        OutputLocationsNumber: Matches.get_one::<String>("Output-Locations").unwrap().parse().expect("Invalid Number Format"),
        OutputFormat: Matches.get_one::<String>("Output-Format").unwrap().to_string(),
    };
    Args.Validate()?;
    #[allow(non_snake_case)]
    let WaitIntervalSeconds = IntervalToSeconds(&Args.WaitInterval)?;

    match Args.OutputFormat.as_str() {
        "YAML" => WriteOutputYaml(&Args),
        "JSON" => WriteOutputJson(&Args),
        _ => WriteOutputTable(&Args)
    }

    Ok(())
}
