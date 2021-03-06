mod chip;
use chip::Cpu;
use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("chip8 <Rom>");
        process::exit(-1)
    }

    let rom_file = &args[1];

    let mut f = match fs::File::open(rom_file) {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(-1);
        }
    };

    //create new cpu (chip 8)
    let mut cpu: Cpu = Cpu::new();

    //load rom into cpu
    cpu.load(&mut f);

    //start execution
    cpu.execute();
}
