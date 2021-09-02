use clap::{App, Arg};
use std::io::{prelude::*, BufReader};
use std::{fs, vec};

mod p6502;
mod tests;

/// Parses the command line arguments. the `addresses` flag is used to ignore the first element
/// of each row and the `INPUT` arguments must contain the path to hex file containing the program.
fn parse_args() -> (bool, String) {
    let matches = App::new("km6502")
        .version("0.1")
        .author("Gonzalo Sastre")
        .about("A (not yet) cycle-accurate MOS 6502 CPU emulator.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the hex input stream")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("addresses")
                .short("a")
                .long("addresses")
                .required(false)
                .help("Pass this flag if the input file's first row consists of addresses.")
                .takes_value(false),
        )
        .get_matches();

    let flags_addresses: bool = matches.is_present("addresses");
    let input_file: String = matches.value_of("INPUT").unwrap().to_string();

    (flags_addresses, input_file)
}

/// Reads a program written in a hex-dump format.
fn read_program(ignore_first_column: bool, path: String, buffer: &mut Vec<u8>) {
    if let Ok(file) = fs::File::open(&path) {
        let buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            match line {
                Ok(line_contents) => {
                    let contents: Vec<&str> = line_contents.split_whitespace().collect();
                    if ignore_first_column {
                        for (index, value) in contents.iter().enumerate() {
                            if index != 0 {
                                buffer.push(u8::from_str_radix(value, 16).unwrap());
                            }
                        }
                    } else {
                        for value in contents.iter() {
                            match u8::from_str_radix(value, 16) {
                                Ok(integer_value) => buffer.push(integer_value),
                                Err(_) => {
                                    println!("Unable to parse an opcode. Did you provide a file which uses addresses without passing the 'addresses' flag?");
                                    std::process::exit(1);
                                }
                            }
                        }
                    }
                }

                Err(error) => {
                    println!("Unable to read line contents. The error is: {}", error);
                    std::process::exit(1);
                }
            }
        }
    } else {
        println!(
            "Unable to open the file '{}'. Does it exist? Do you have permission to read it?",
            &path
        );
        std::process::exit(1);
    }
}

fn main() {
    let (flags_addresses, input_file): (bool, String) = parse_args(); // Reads the command line arguments.
    let mut cpu: p6502::P6502 = p6502::P6502::default(); // Creates an new processor instance.
    cpu.reset(); // Sets the correct initial values.

    /* Reads the program and loads its contents into memory. */
    let mut program: Vec<u8> = vec![];
    read_program(flags_addresses, input_file, &mut program);
    let memory: p6502::Memory = p6502::Memory::from_program_vec(program);
    cpu.set_memory(memory);

    cpu.execute(); // Executes the program.

    /* Runs the interactive prompt once the program is finished. */
    println!(
        "The program finished at PC=${:04x}. The interactive prompt will now appear.",
        cpu.pc
    );
    cpu.interactive();
}
