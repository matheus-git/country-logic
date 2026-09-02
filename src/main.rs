mod query_countries;

use clap::{Parser, Subcommand};
use scryer_prolog::{LeafAnswer, Machine, MachineBuilder, QueryState, Term};
use std::collections::BTreeSet;
use std::fs;

use crate::query_countries::Country;

#[derive(Parser)]
#[command(
    name = "country-logic",
    about = "Query country data with four Prolog rules"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all available countries.
    Countries,
    /// List all available regions.
    Regions,
    /// List countries with more than 100 million inhabitants.
    Large,
    /// List countries in a region.
    Region { region: String },
    /// List countries more populous than the selected country.
    MorePopulous { country: String },
    /// List countries that share a land border with the selected country.
    Neighbors { country: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let command = Cli::parse().command;
    let countries = query_countries::fetch_countries().await?;

    match command {
        Command::Countries => return print_countries(&countries),
        Command::Regions => return print_regions(&countries),
        _ => {}
    }

    let mut machine = build_machine()?;
    populate_machine(&mut machine, &countries)?;

    let query = match command {
        Command::Countries | Command::Regions => unreachable!(),
        Command::Large => "large_country(Country).".to_owned(),
        Command::Region { region } => {
            format!("country_in_region(Country, {}).", prolog_atom(&region))
        }
        Command::MorePopulous { country } => format!(
            "more_populous(Country, {}).",
            prolog_atom(&resolve_country(&countries, &country)?.name)
        ),
        Command::Neighbors { country } => format!(
            "borders({}, Neighbor).",
            prolog_atom(&resolve_country(&countries, &country)?.name)
        ),
    };

    print_answers(machine.run_query(query))
}

fn print_countries(countries: &[Country]) -> Result<(), Box<dyn std::error::Error>> {
    let names = countries
        .iter()
        .map(|country| country.name.as_str())
        .collect::<BTreeSet<_>>();
    print_list(names)
}

fn print_regions(countries: &[Country]) -> Result<(), Box<dyn std::error::Error>> {
    let regions = countries
        .iter()
        .map(|country| country.region.as_str())
        .collect::<BTreeSet<_>>();
    print_list(regions)
}

fn print_list<'a>(
    values: impl IntoIterator<Item = &'a str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut found = false;
    for value in values {
        found = true;
        println!("{value}");
    }
    if found {
        Ok(())
    } else {
        Err("no data returned by the API".into())
    }
}

fn build_machine() -> Result<Machine, Box<dyn std::error::Error>> {
    let mut machine = MachineBuilder::new().build();
    let rules = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/rules.pl"))?;
    machine.consult_module_string("user", rules);
    Ok(machine)
}

fn populate_machine(
    machine: &mut Machine,
    countries: &[Country],
) -> Result<(), Box<dyn std::error::Error>> {
    for country in countries {
        let name = prolog_atom(&country.name);
        for fact in [
            format!("population({name}, {})", country.population),
            format!("region({name}, {})", prolog_atom(&country.region)),
            format!("alpha3({name}, {})", prolog_atom(&country.alpha3)),
        ] {
            assert_fact(machine, &fact)?;
        }
        for border in &country.borders {
            assert_fact(
                machine,
                &format!("border_code({name}, {})", prolog_atom(border)),
            )?;
        }
    }
    Ok(())
}

fn assert_fact(machine: &mut Machine, fact: &str) -> Result<(), Box<dyn std::error::Error>> {
    let succeeded = machine
        .run_query(format!("assertz({fact})."))
        .any(|answer| matches!(answer, Ok(LeafAnswer::True)));
    succeeded
        .then_some(())
        .ok_or_else(|| format!("failed to insert Prolog fact: {fact}").into())
}

fn resolve_country<'a>(
    countries: &'a [Country],
    input: &str,
) -> Result<&'a Country, Box<dyn std::error::Error>> {
    countries
        .iter()
        .find(|country| {
            country.name.eq_ignore_ascii_case(input) || country.alpha3.eq_ignore_ascii_case(input)
        })
        .ok_or_else(|| format!("country not found: {input}").into())
}

fn print_answers(query: QueryState<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let mut found = false;
    for answer in query {
        match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => {
                found = true;
                for (variable, value) in bindings {
                    println!("{variable} = {}", format_term(&value));
                }
            }
            Ok(LeafAnswer::True) => {
                found = true;
                println!("true");
            }
            Ok(LeafAnswer::False) => {}
            Ok(LeafAnswer::Exception(error)) => {
                return Err(format!("Prolog exception: {error:?}").into());
            }
            Err(error) => return Err(format!("Prolog error: {error:?}").into()),
        }
    }
    if !found {
        println!("No results.");
    }
    Ok(())
}

fn format_term(term: &Term) -> String {
    match term {
        Term::Atom(value) | Term::String(value) | Term::Var(value) => value.clone(),
        Term::Integer(value) => value.to_string(),
        other => format!("{other:?}"),
    }
}

fn prolog_atom(value: &str) -> String {
    format!("'{}'", value.replace('\\', "\\\\").replace('\'', "''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_machine() -> Machine {
        let mut machine = MachineBuilder::new().build();
        machine.consult_module_string("user", include_str!("../rules.pl"));
        for fact in [
            "population('Smallland', 5000000)",
            "region('Smallland', 'Test Region')",
            "alpha3('Smallland', 'SML')",
            "border_code('Smallland', 'BIG')",
            "population('Bigland', 150000000)",
            "region('Bigland', 'Test Region')",
            "alpha3('Bigland', 'BIG')",
        ] {
            assert_fact(&mut machine, fact).unwrap();
        }
        machine
    }

    fn has_solution(machine: &mut Machine, query: &str) -> bool {
        machine
            .run_query(query)
            .any(|answer| matches!(answer, Ok(LeafAnswer::True | LeafAnswer::LeafAnswer { .. })))
    }

    #[test]
    fn supports_all_four_rules() {
        let mut machine = test_machine();
        assert!(has_solution(&mut machine, "large_country('Bigland')."));
        assert!(has_solution(
            &mut machine,
            "country_in_region('Smallland', 'Test Region')."
        ));
        assert!(has_solution(
            &mut machine,
            "more_populous('Bigland', 'Smallland')."
        ));
        assert!(has_solution(
            &mut machine,
            "borders('Smallland', 'Bigland')."
        ));
    }

    #[test]
    fn escapes_prolog_atoms() {
        assert_eq!(prolog_atom("Cote d'Ivoire"), "'Cote d''Ivoire'");
    }

    #[test]
    fn creates_sorted_unique_lists() {
        let countries = [
            Country {
                name: "Zedland".to_owned(),
                alpha3: "ZED".to_owned(),
                population: 1,
                region: "West".to_owned(),
                borders: vec![],
            },
            Country {
                name: "Alphaland".to_owned(),
                alpha3: "ALP".to_owned(),
                population: 1,
                region: "West".to_owned(),
                borders: vec![],
            },
        ];

        let regions = countries
            .iter()
            .map(|country| country.region.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(regions.into_iter().collect::<Vec<_>>(), ["West"]);
    }
}
