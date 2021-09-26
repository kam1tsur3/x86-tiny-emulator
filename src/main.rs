use std::fs;

// lib internal
use general_registers::GeneralRegisters;

const MEMORY_SIZE: usize = 0x10000;
const ADDRESS_LOAD: usize = 0x7c00;
struct ModRM {
    mod_internal: u8,
    op_or_reg: u8,
    rm: u8,
    sib: u8,
    //disp: DISP,
    disp8: i8,
    disp32: u32,
}
//enum DISP {
//    disp8(i8),
//    disp32(u32),
//}
//enum OPorREGINDEX {
//    opecode(u8),
//    reg_index(u8),
//}

struct Emulator {
    regs: general_registers::GeneralRegisters,
    eflags: u32,
    memory: Vec<u8>,
    eip: u32,
}

impl Emulator {
    fn new () -> Emulator {
        let mut regs = general_registers::GeneralRegisters::new();
        let mut pre : Vec<u8> = vec![0;ADDRESS_LOAD];
        let mut mem = fs::read("./exec-modrm-test/test.raw").unwrap();
        let mut padding: Vec<u8> = vec![0;(MEMORY_SIZE-mem.len() - ADDRESS_LOAD)];
        mem.append(&mut padding);
        pre.append(&mut mem);
        println!("Memory length: {:x}", pre.len());
        Emulator {
            regs: regs,
            eflags: 0,
            memory: pre,
            eip: ADDRESS_LOAD as u32,
        }
    }

    fn exec(&mut self) {
        let code = self.get_unsign_code8(0); 
        match code {
            0x01 => self.add_rm32_r32(),
            0x83 => self.code_83(),
            0x89 => self.mov_rm32_r32(),
            0x8b => self.mov_r32_rm32(),
            0xb8 | 0xb9 | 0xba | 0xbb |
            0xbc | 0xbd | 0xbe | 0xbf => self.mov_r32_imm32(),
            0xc7 => self.mov_rm32_imm32(),
            0xe9 => self.near_jump(),
            0xeb => self.short_jump(),
            0xff => self.code_ff(),
            _ => self.not_impl(),
        }
    }
    // sub function
    fn parse_modrm(&mut self) -> ModRM {
        // ●●●EIPがどの状態で入ってるのか！！！
        let code = self.get_unsign_code8(0); 
        let mut ret = ModRM {
            mod_internal: (code & 0xc0) >> 6,
            op_or_reg: (code & 0x38) >> 3,
            rm: (code & 0x7),
            sib: 0,
            disp8: 0,
            disp32: 0,
        };
        self.eip += 1;

        // sib enable/disable
        if ret.mod_internal != 3 && ret.rm == 4 {
            ret.sib = self.get_unsign_code8(0); 
            self.eip += 1;
        }

        if (ret.mod_internal == 0 && ret.mod_internal == 5) || ret.mod_internal == 2 {
            ret.disp32 = self.get_unsign_code32(0); 
            self.eip += 4;
        } else if ret.mod_internal == 1 {
            ret.disp8 = self.get_sign_code8(0); 
            self.eip += 1;
        }

        return ret;
    }

    fn set_rm32(&mut self, modrm: &ModRM, value: u32) {
        if modrm.mod_internal == 3 {
            self.set_reg32_by_idx(modrm.rm, value);
        } else {
            let addr = self.calc_memory_address(&modrm);
            self.write_mem_u32(addr, value); 
        }
    }

    fn get_rm32(&self, modrm: &ModRM) -> u32{
        if modrm.mod_internal == 3 {
            return self.get_reg32_by_idx(modrm.rm);
        } else {
            let addr = self.calc_memory_address(&modrm);
            return self.read_mem_u32(addr); 
        }
    }

    fn calc_memory_address(&self, modrm: &ModRM) -> u32 {
        match (modrm.mod_internal, modrm.rm) {
            (0, 4) => {
                println!("Not implemented in calc_memory_address()");
                return 0; 
            },
            (0, 5) => {
                return modrm.disp32;
            },
            (0, _) => {
                return self.get_reg32_by_idx(modrm.rm);
            },
            (1, 4) => {
                println!("Not implemented in calc_memory_address()");
                return 0; 
            },
            (1, _) => {
                return ((self.get_reg32_by_idx(modrm.rm) as i32) + (modrm.disp8 as i32)) as u32;
            },
            (2, 4) => {
                println!("Not implemented in calc_memory_address()");
                return 0; 
            },
            (2, _) => {
                return ((self.get_reg32_by_idx(modrm.rm) as i32) + (modrm.disp32 as i32)) as u32;

            },
            (_, _) => {
                println!("Not implemented in calc_memory_address()");
                return 0; 
            }
        }
    }

    fn get_reg32_by_idx(&self, idx: u8) -> u32 {
        match idx{
            general_registers::EAX_INDEX => self.regs.eax,
            general_registers::ECX_INDEX => self.regs.ecx,
            general_registers::EDX_INDEX => self.regs.edx,
            general_registers::EBX_INDEX => self.regs.ebx,
            general_registers::ESP_INDEX => self.regs.esp,
            general_registers::EBP_INDEX => self.regs.ebp,
            general_registers::ESI_INDEX => self.regs.esi,
            general_registers::EDI_INDEX => self.regs.edi,
            _ => {
                println!("Not implemented in get_reg32_by_idx()");
                return 0;
            }
        }
    }

    fn set_reg32_by_idx(&mut self, idx: u8, value: u32) {
         match idx{
            general_registers::EAX_INDEX => self.regs.eax = value,
            general_registers::ECX_INDEX => self.regs.ecx = value,
            general_registers::EDX_INDEX => self.regs.edx = value,
            general_registers::EBX_INDEX => self.regs.ebx = value,
            general_registers::ESP_INDEX => self.regs.esp = value,
            general_registers::EBP_INDEX => self.regs.ebp = value,
            general_registers::ESI_INDEX => self.regs.esi = value,
            general_registers::EDI_INDEX => self.regs.edi = value,
            _ => {
                println!("Not implemented in get_reg32_by_idx()");
            }
        }
    }
    // instruction
    fn not_impl(&mut self) {
       println!("Not implemented instruction");
    }

    fn add_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm = self.parse_modrm();
        let value: u32 = self.get_reg32_by_idx(modrm.op_or_reg);
        let org: u32 = self.get_rm32(&modrm); 
        self.set_rm32(&modrm ,org+value);
    }

    fn code_83(&mut self) {
        self.eip += 1;
        let mut modrm = self.parse_modrm();
        match modrm.op_or_reg {
            5 => self.sub_rm32_imm8(&modrm),
            _ => println!("Not implemented in code_83()"),
        }
    }

    fn sub_rm32_imm8(&mut self, modrm: &ModRM) {
        let rm32 = self.get_rm32(&modrm);
        let imm8 = self.get_sign_code8(0);
        self.eip += 1;
        self.set_rm32(&modrm, rm32 - ((imm8 as i32) as u32));
    }

    fn code_ff(&mut self) {
        self.eip += 1;
        let mut modrm = self.parse_modrm();
        match modrm.op_or_reg {
            0 => self.inc_rm32(&modrm),
            _ => println!("Not implemened: in code_ff()")
        }
    }

    fn inc_rm32(&mut self, modrm: &ModRM) {
        let value = self.get_rm32(&modrm);
        self.set_rm32(&modrm, value+1);
    }

    fn mov_r32_rm32(&mut self) {
        self.eip += 1;
        let modrm: ModRM = self.parse_modrm();
        let reg_idx = modrm.op_or_reg; // to avoid move error
        let value = self.get_rm32(&modrm);
        self.set_reg32_by_idx(reg_idx, value);
    }

    fn mov_rm32_r32(&mut self) {
        self.eip += 1;
        let modrm: ModRM = self.parse_modrm();
        let value = self.get_reg32_by_idx(modrm.op_or_reg);
        self.set_rm32(&modrm, value);
    }

    fn mov_r32_imm32(&mut self) {    
        let reg = self.get_unsign_code8(0);
        let value = self.get_unsign_code32(1);
        match (reg - 0xb8) {
            general_registers::EAX_INDEX =>
                self.regs.eax = value,
            general_registers::ECX_INDEX =>
                self.regs.ecx = value,
            general_registers::EDX_INDEX =>
                self.regs.edx = value,
            general_registers::EBX_INDEX =>
                self.regs.ebx = value,
            general_registers::ESP_INDEX =>
                self.regs.esp = value,
            general_registers::EBP_INDEX =>
                self.regs.ebp = value,
            general_registers::ESI_INDEX =>
                self.regs.esi = value,
            general_registers::EDI_INDEX =>
                self.regs.edi = value,
            _ => {
                println!("[*]Error in mov_r32_imm32()");
                println!("value = {}", value)
            },
        }
        self.eip += 5;
    }

    fn mov_rm32_imm32(&mut self) {
        self.eip += 1;
        let modrm: ModRM = self.parse_modrm();
        let value = self.get_unsign_code32(0);
        self.eip += 4;
        self.set_rm32(&modrm, value);
    }

    fn near_jump(&mut self) {
        let diff = self.get_sign_code32(1);
        self.eip = ((self.eip as i32) + diff + 5) as u32; // 2 is length of this instruction
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        self.eip = ((self.eip as i32) + (diff as i32) + 2) as u32; // 2 is length of this instruction
    }

    // memory operation
    fn get_sign_code8(&self, offset: u32) -> i8{
        self.get_unsign_code8(offset) as i8
    }

    fn get_unsign_code8(&self, offset: u32) -> u8{
        self.read_mem_u8(self.eip+offset)
    }

    fn get_sign_code32(&self, offset: u32) -> i32{
        self.get_unsign_code32(offset) as i32
    }

    fn get_unsign_code32(&self, offset: u32) -> u32{
        self.read_mem_u32(self.eip+offset)
    }
    fn read_mem_u8(&self, offset:u32) -> u8 {
        //let index = (self.eip + offset) as usize;
        //self.memory[index]
        self.memory[offset as usize]
    }

    fn read_mem_i8(&self, offset:u32) -> i8 {
        self.read_mem_u8(offset) as i8
    }
    
    fn read_mem_u32(&self, offset:u32) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.memory[(i+offset) as usize] as u32) << (i*8);
        }
        return ret;
    }

    fn read_mem_i32(&self, offset:u32) -> i32 {
        self.read_mem_u32(offset) as i32
    }

    fn write_mem_u8(&mut self, offset:u32, value: u8) {
        self.memory[offset as usize] = value;
    }

    fn write_mem_i8(&mut self, offset:u32, value:i8) {
        self.write_mem_u8(offset, value as u8);
    }

    fn write_mem_u32(&mut self, offset: u32, value: u32) {
        let mut c:u8;
        for i in 0..4 {
            c = ((value >> (i*8)) & 0xff) as u8;
            self.memory[(offset+i)as usize] = c;
        }
    }

    fn write_mem_i32(&mut self, offset: u32, value: i32) {
        self.write_mem_u32(offset, value as u32);
    }
}

fn main() {
    let mut emulator = Emulator::new();
    emulator.regs.print_regs();
    println!("eip: 0x{:x}", emulator.eip);
    //for _ in 0..2 {
    while true {
        emulator.exec();
        println!("eip: 0x{:x}", emulator.eip);
        if emulator.eip == 0 {
            break
        }
    }
    emulator.regs.print_regs();
    println!("Exit, world!");
}
