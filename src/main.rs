use scryer_prolog::{LeafAnswer, MachineBuilder};
use scryer_prolog::Term::{String, Atom};
use std::fs;

fn main() {
    let mut machine = MachineBuilder::new().build();

    let rules = fs::read_to_string("swiss.pl")
        .expect("error");

    machine.consult_module_string("rules", rules);

    let query = machine.run_query("rodada(X, 4.5, 7).");

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
                println!("verdadeiro");
            }

            Ok(LeafAnswer::False) => {
                println!("fim das soluções");
            }

            Ok(LeafAnswer::Exception(e)) => {
                println!("erro Prolog: {:?}", e);
            }

            Err(e) => {
                println!("erro Rust: {:?}", e);
            }
        }
    });
}
