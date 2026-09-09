use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "strmul", about = "CLI-tool to multiply strings")]
struct Args {
    string: String,
    count: usize,

    #[arg(long, short)]
    join: Option<String>,

    #[arg(long)]
    head: bool,

    #[arg(long, short)]
    out: Option<PathBuf>,
}

fn main() {
    let args=  Args::parse();
    
    let mut string = args.string;
    if let Some(join) = args.join {
        string += join.as_str();
    }
    if args.head {
        string += "\n";
    }

    if let Some(out) = args.out {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(out).expect("Couldn't open file");
        write!(file, "{}", string.repeat(args.count)).expect("Couldn't write to file");
    } else {
        print!("{}", string.repeat(args.count));
    }
}
