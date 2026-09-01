mod query_countries;

use scryer_prolog::{LeafAnswer, MachineBuilder, Machine, QueryState};
use scryer_prolog::Term::{String, Atom};
use std::fs;

use crate::query_countries::Country;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let countries = query_countries::query_countries().await?;
    
    let mut machine = MachineBuilder::new().build();
    let rules = fs::read_to_string("rules.pl")
        .expect("error");

    machine.consult_module_string("rules", rules);
    populate_country(&mut machine, countries);
    test_prolog(machine);

    Ok(())
}

fn populate_country(machine: &mut Machine, countries: Vec<Country>) {
    for country in countries {
        let formatted_name = country.name.replace('\'', "''");
        let query = machine.run_query(format!(
            "assertz(country('{}')).", formatted_name
        ));
        query_result(query);

        let query = machine.run_query(format!(
            "assertz(population('{}', {})).", formatted_name, country.population
        ));
        query_result(query);

        let formatted_region = country.region.replace('\'', "''");
        let query = machine.run_query(format!(
            "assertz(region('{}', '{}')).", formatted_name, formatted_region
        ));
        query_result(query);
    }
}

#[allow(dead_code)]
fn test_prolog(mut machine: Machine) {
    println!("");
    let query = machine.run_query("large_country(X).");
    query_result(query);
}

#[allow(dead_code)]
fn query_result(query: QueryState<'_>) {
    query.for_each(|answer| {
        match answer {
            Ok(LeafAnswer::LeafAnswer{ bindings, .. }) => {
                for (_, term) in bindings {
                    match term {
                        Atom(value) => {
                            println!("{value}");
                        },
                        String(value) => {
                            println!("{value}");
                        },
                        _ => {
                            println!("{:?}", term);
                        }
                    }
                }
            }

            Ok(LeafAnswer::True) => {
                println!("true");
            }

            Ok(LeafAnswer::False) => {
                println!("false");
            }

            Ok(LeafAnswer::Exception(e)) => {
                println!("{:?}", e);
            }

            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    });
}
