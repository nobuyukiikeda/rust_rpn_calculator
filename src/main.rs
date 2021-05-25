use std::{fs::File, io::{BufRead, BufReader, stdin}, path::PathBuf};

use anyhow::Result;
use clap::Clap;
mod rpn_calculator;

#[derive(Clap, Debug)]
#[clap(
    name = "RPN program",
    version = "1.0.0",
    author = "Nobuyuki Ikeda",
    about = "RPN calculator"
)]

struct Opts {
    // Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = rpn_calculator::RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ok() {
        assert_eq!(2 * 2, 4);
    }
}
