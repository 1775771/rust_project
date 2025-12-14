use std::env;
use std::process;
use std::path::Path;
use std::fs::{self, File};
use std::io::Read;

struct ElfIdent {
    magic: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    abi: u8,
    abi_version: u8,
    padding: [u8; 7],
}

impl ElfIdent {
    fn from_bytes(bytes: &[u8]) -> Self {
        ElfIdent {
            magic: [bytes[0], bytes[1], bytes[2], bytes[3]],
            class: bytes[4],
            data: bytes[5],
            version: bytes[6],
            abi: bytes[7],
            abi_version: bytes[8],
            padding: [
                bytes[9], bytes[10], bytes[11],
                bytes[12], bytes[13], bytes[14], bytes[15],
            ],
        }
    }

    fn is_elf(&self) -> bool {
        self.magic == [0x7f, 0x45, 0x4c, 0x46]
    }

    fn class_str(&self) -> &'static str {
        match self.class {
            1 => "ELF 32-bit",
            2 => "ELF 64-bit",
            _ => "Unknown ELF class",
        }
    }

    fn endianness_str(&self) -> &'static str {
        match self.data {
            1 => "Little endian",
            2 => "Big endian",
            _ => "Unknown endianness",
        }
    }

    fn abi_str(&self) -> &'static str {
        match self.abi {
            0 => "System V",
            3 => "Linux",
            6 => "Solaris",
            9 => "FreeBSD",
            _ => "Unknown ABI",
        }
    }
}

fn check_file<T: AsRef<Path>>(file: T) {
    let filetype = fs::metadata(&file);
    let path = file.as_ref();

    match filetype {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("{:?} is a file", path);
            } else if metadata.is_dir() {
                println!("{:?} is a directory.", path);
                process::exit(1);
            }
        } Err(e) => {
            println!("Error {}", e);
            process::exit(1);
        }
    }
}

fn check_args(args: &Vec<String>){
    if args.len() == 1 || (args.len() == 2 && &args[1] == "h") {
        println!("How to use:\n");
        println!("cargo run [file]");
        process::exit(1);
    } else if args.len() != 2 {
        println!("Need only a file.");
        process::exit(1);
    }

    if !Path::new(&args[1]).exists() {
        println!("Need an existing file.");
        process::exit(1);
    }

    check_file(&args[1]);
}

fn read_ident(file: &String) -> Vec<u8>{
    let mut f = File::open(file).unwrap_or_else(|e| {
        println!("Failed to open file {}", e);
        process::exit(1);
    });
    
    let mut ident = vec![0u8; 16];

    let byte_read = f.read(&mut ident).unwrap_or_else(|e|{
        println!("Failed to read ELF identifier: {}", e);
        process::exit(1);
    });

    ident.truncate(byte_read);

    if ident.len() == 16 {
        return ident;
    }

    println!("The file doesn't have enough header bytes (16 minimum) to be analysed.");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);

    let ident = read_ident(&args[1]);

    let elf_ident = ElfIdent::from_bytes(&ident);

    if !elf_ident.is_elf() {
        println!("{:?} is not an ELF file.", &args[1]);
        process::exit(1);
    }

    println!("{:?} is an ELF file.", &args[1]);
    println!("Class       : {}", elf_ident.class_str());
    println!("Endianness  : {}", elf_ident.endianness_str());
    println!("ABI         : {}", elf_ident.abi_str());
    println!("ABI version : {}", elf_ident.abi_version);
}
