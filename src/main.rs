mod chip;
use chip::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    // cpu.load("rom file");
    cpu.execute();
}