mod query_countries;

use scryer_prolog::{LeafAnswer, MachineBuilder, QueryState};
use scryer_prolog::Term::{String, Atom};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    query_countries::query_countries().await?;

    Ok(())
}

#[allow(dead_code)]
fn test_prolog() {
    let mut machine = MachineBuilder::new().build();

    let rules = fs::read_to_string("rules.pl")
        .expect("error");

    machine.consult_module_string("rules", rules);

    let query = machine.run_query("assertz(country(angola)).");
    query_result(query);

    println!("");
    let query = machine.run_query("country(X).");
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
