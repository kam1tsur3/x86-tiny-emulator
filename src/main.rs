use std::fs;

// lib internal
use general_registers::GeneralRegisters;

const MEMORY_SIZE: usize = 0x1000;

struct Emulator {
    regs: general_registers::GeneralRegisters,
    eflags: u32,
    memory: Vec<u8>,
    eip: u32,
}

impl Emulator {
    fn new () -> Emulator {
        let mut regs = general_registers::GeneralRegisters::new();
        Emulator {
            regs: regs,
            eflags: 0,
            memory: fs::read("memory.raw").unwrap(),
            eip: 0,
        }
    }

    fn exec(&mut self) {
        let code = self.read_mem_u8(0);
        match code {
            0xb8 | 0xb9 | 0xba | 0xbb |
            0xbc | 0xbd | 0xbe | 0xbf => self.mov_r32_imm32(),
            0xeb => self.short_jump(),
            _ => self.not_impl(),
        }
    }

    // instruction
    fn not_impl(&mut self) {
       println!("Not implemented");
    }

    fn mov_r32_imm32(&mut self) {    
        let reg = self.read_mem_u8(0);
        let value = self.read_mem_u32(1);
        match (reg - 0xb8) as u32 {
            general_registers::EAX_INDEX =>
                self.regs.eax = value,
            general_registers::EBX_INDEX =>
                self.regs.ebx = value,
            general_registers::ECX_INDEX =>
                self.regs.ecx = value,
            general_registers::EDX_INDEX =>
                self.regs.edx = value,
            general_registers::ESI_INDEX =>
                self.regs.esi = value,
            general_registers::EDI_INDEX =>
                self.regs.edi = value,
            general_registers::EBP_INDEX =>
                self.regs.ebp = value,
            general_registers::ESP_INDEX =>
                self.regs.esp = value,
            _ => {
                println!("[*]Error in mov_r32_imm32()");
                println!("value = {}", value)
            },
        }
        self.eip += 5;
    }

    fn short_jump(&mut self) {
        let diff = self.read_mem_i8(1);
        self.eip = ((self.eip as i32) + (diff as i32) + 2) as u32; // 2 is length of this instruction
    }

    // memory operation
    fn read_mem_i8(&self, offset:i32) -> i8 {
        let index = ((self.eip as i32) + offset) as usize;
        self.memory[index] as i8
    }
    
    fn read_mem_u8(&self, offset:i32) -> u8 {
        let index = ((self.eip as i32) + offset) as usize;
        self.memory[index]
    }

    fn read_mem_u32(&self, offset:i32) -> u32 {
        let index = (self.eip as i32) + offset;
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.memory[(i+index) as usize] as u32) << (i*8);
        }
        return ret;
    }
}


// test
fn f1(i: i32) -> i32 {
    i + 1
}

fn f2(i: i32) -> i32 {
    i + 2
}

static tables: [fn(i32) -> i32;2] = [f1, f2];

fn main() {
    let mut emulator = Emulator::new();
    emulator.regs.print_regs();
    println!("eip: {}", emulator.eip);
    while true {
        emulator.exec();
        if emulator.eip == 0 {
            break
        }
    }
    emulator.regs.print_regs();
    println!("eip: {}", emulator.eip);
    println!("Exit, world!");
}
