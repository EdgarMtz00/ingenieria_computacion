use analizador_sintactico::syntax_analysis::syntax_analysis;
use clap::Parser;

/// Analizador léxico y sintáctico para expresiones matematicas
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Archivo conteniendo la expresión matemática
    #[arg(short, long)]
    src: String,
}
fn main() {
    let args = Args::parse();
    println!("{}",syntax_analysis(&args.src));
}