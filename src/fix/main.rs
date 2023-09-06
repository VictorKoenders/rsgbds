use std::{fs::File, io::Read};

use crate::args::MBCType;

mod args;

fn main() {
    let args = args::parse();

    if args.version {
        println!("Version 0.0.0");
        return;
    }
    let mut source = match args.filename.as_str() {
        "-" => Source::StdinStdout(std::io::stdin().lock(), std::io::stdout().lock()),
        _ => Source::File(File::open(args.filename.as_str()).expect("Could not open file")),
    };

    const BANK_SIZE: usize = 0x4000;
    let mut rom0 = [0u8; BANK_SIZE];
    source
        .reader()
        .read_exact(&mut rom0)
        .expect("Failed to read");
    let header_size = if args.mbc_type.map_or(false, |m| m.ty == MBCType::TPP1) {
        0x154
    } else {
        0x150
    };

    todo!()
}

enum Source<'a> {
    StdinStdout(std::io::StdinLock<'a>, std::io::StdoutLock<'a>),
    File(File),
}

impl Source<'_> {
    fn parse_file(mut self, args: args::Args) {}

    fn reader(&mut self) -> &mut dyn std::io::Read {
        match self {
            Self::File(file) => file,
            Self::StdinStdout(stdin, _) => stdin,
        }
    }
    fn writer(&mut self) -> &mut dyn std::io::Write {
        match self {
            Self::File(file) => file,
            Self::StdinStdout(_, stdout) => stdout,
        }
    }

    fn read_exact(&mut self, out: &mut [u8]) -> std::io::Result<()> {
        match self {
            Self::File(f) => f.read_exact(out),
            Self::StdinStdout(stdin, _) => stdin.read_exact(out),
        }
    }
}
