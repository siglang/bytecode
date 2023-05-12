mod code;

use bytecode::{
    rawbytes::RawBytes,
    vm::{Instructions, Program, Vm},
};
use clap::{Parser, Subcommand};
use code::Code;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(
    bin_name = "bytecode",
    version = "0.0.0",
    arg_required_else_help = true
)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(name = "b2r", about = "ByteCode source code file -> RawBytes file")]
    B2R {
        #[clap(short, long)]
        input: PathBuf,
        #[clap(short, long)]
        output: PathBuf,
    },
    #[clap(name = "interpret", about = "Interpret RawBytes file")]
    Interpret {
        #[clap(short, long)]
        input: PathBuf,
    },
    #[clap(name = "run", about = "Run ByteCode source code file")]
    Run {
        #[clap(short, long)]
        input: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        SubCommand::B2R { input, output } => {
            let bytecode = fs::read_to_string(input).unwrap();
            let parsed = Code(&bytecode).parse().0;
            let rawbytes: RawBytes = Instructions(parsed).into();

            fs::write(output, rawbytes.0).unwrap();
        }
        SubCommand::Interpret { input } => {
            let contents = fs::read(input).unwrap();
            let rawbytes = RawBytes(&contents);
            let instructions: Instructions = rawbytes.try_into().unwrap();
            let program = Program(instructions);

            Vm::new(program).run().unwrap();
        }
        SubCommand::Run { input } => {
            let bytecode = fs::read_to_string(input).unwrap();
            let parsed = Code(&bytecode).parse().0;
            let rawbytes: RawBytes = Instructions(parsed).into();
            let instructions: Instructions = rawbytes.try_into().unwrap();
            let program = Program(instructions);

            Vm::new(program).run().unwrap();
        }
    }
}
