use clap::{ Parser, arg };

#[derive(Parser, Debug)]
#[command(name = "strmul", about = "CLI-tool to multiply strings")]
struct Args {
    string: String,
    count: usize,

    #[arg(long, short)]
    join: Option<String>,

    #[arg(long)]
    head: bool,
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
    
    print!("{}", string.repeat(args.count));
}
