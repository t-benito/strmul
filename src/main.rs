use clap::{ Parser, arg, command, ArgGroup };

#[derive(Parser, Debug)]
#[command(name = "strmul", about = "CLI-tool to multiply strings")]
struct Args {
    string: String,
    count: usize,

    #[arg(long)]
    head: bool,
}

fn main() {
    let args=  Args::parse();
    if args.head {
        for _ in 0..args.count { println!("{}", args.string) }
    } else {
        for _ in 0..args.count { print!("{}", args.string) }
    }
}
