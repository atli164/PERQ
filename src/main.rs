use perq::oeis;
use std::io::BufRead;
use perq::{Series, PowerSeries, Matrix, lll};
use perq::runtime;
use rug::Integer;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut bufin = std::io::BufReader::new(stdin);
    let mut rt = runtime::RunTimeEnvironment::new("stripped".to_string()).unwrap();
    rt.repl(&mut bufin, &mut stdout, true);
}
