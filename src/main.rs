mod file_parsing;
use file_parsing::*;

mod production_map;
use production_map::*;

mod verifier;
use verifier::*;

fn input() -> std::io::Result<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(s)
}

fn main() -> std::io::Result<()> {
    let mut productions = ProductionMap::new();
    for file in std::env::args()
        .skip(1)
        .map(|f|
            std::fs::read_to_string(&f)
        ) {
        parse_file(file?, &mut productions);
    }
    let input = input()?;
    
    verify(&input, &mut productions);

    Ok(())
}

