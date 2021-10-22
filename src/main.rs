use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "yu",
    about = "Super awesome sample RPN calculator"
)]
struct Opt {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,

}

fn main() {
    let _opt = Opt::parse();


}
