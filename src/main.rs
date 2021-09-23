use general_registers::GeneralRegisters;

const MEMORY_SIZE = 0x1000

struct Emulator {
    regs: general_registers::GeneralRegisters,
    eflags: u32,
    memory: &[u8; MEMORY_SIZE],
    eip: u32,
}

impl Emulator {
    fn new () -> Emulator {
        let mut regs = general_registers::GeneralRegisters::new();
        Emulator {
            regs: regs,
            eflags: 0,
            memory: [0;MEMORY_SIZE],
            eip: 0,
        }
    }
}
fn main() {
    let mut emulator = Emulator::new();
    emulator.regs.print_regs();
    emulator.regs.eax = 10;
    emulator.regs.print_regs();
    println!("Hello, world!");
}
