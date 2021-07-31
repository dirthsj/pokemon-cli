use clap::{App, Arg, ArgMatches, SubCommand};
use serde::Deserialize;
use std::fmt::Formatter;

#[derive(Debug)]
struct ValidationError(String);

impl std::error::Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a validation error: {}", self.0)
    }
}

#[derive(Debug, Deserialize)]
struct PokemonListResponseEntry {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct PokemonListResponse {
    count: i32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<PokemonListResponseEntry>,
}

type StandardResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> StandardResult<()> {
    let list_command = SubCommand::with_name("list")
        .arg(
            Arg::with_name("offset")
                .short("o")
                .long("offset")
                .takes_value(true)
                .default_value("0")
                .validator(|x| {
                    if x.parse::<u32>().is_ok() {
                        Ok(())
                    } else {
                        Err("Must be a positive integer".into())
                    }
                })
                .help("The index of the first pokemon to return"),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .default_value("10")
                .validator(|x| {
                    if x.parse::<u32>().is_ok() {
                        Ok(())
                    } else {
                        Err("Must be a positive integer".into())
                    }
                })
                .help("The maximum number of pokemon to return"),
        )
        .about("Lists pokemon");
    let app = App::new("pokemon-cli")
        .subcommand(list_command)
        .version("0.1-prerelease")
        .about("Browse pokemon from the command line")
        .author("Steven Dirth");
    let matches = app.get_matches();
    let (command_option, matches_command) = matches.subcommand();
    match command_option {
        "list" => println!("{:#?}", list_pokemon_subcommand(matches_command).await?),
        _ => eprintln!("No such command"),
    }
    Ok(())
}

async fn list_pokemon_subcommand(args: Option<&ArgMatches<'_>>) -> StandardResult<()> {
    let args = args.unwrap();
    let offset = args
        .value_of("offset")
        .unwrap_or("-1")
        .parse::<i32>()
        .unwrap_or(-1);
    if offset < 0 {
        return Err(Box::new(ValidationError("Invalid Offset".into())));
    }
    let limit = args
        .value_of("limit")
        .unwrap_or("-1")
        .parse::<i32>()
        .unwrap_or(-1);
    if limit < 0 {
        return Err(Box::new(ValidationError("Invalid Limit".into())));
    }
    println!("{:#?}", list_pokemon(offset, limit).await?);
    Ok(())
}

async fn list_pokemon(offset: i32, limit: i32) -> StandardResult<PokemonListResponse> {
    let client = reqwest::Client::default();
    let request = client
        .get("https://pokeapi.co/api/v2/pokemon/")
        .query(&[("offset", offset), ("limit", limit)]);
    let response = request.send().await?;
    let response_json = response.json::<PokemonListResponse>().await?;
    Ok(response_json)
}
