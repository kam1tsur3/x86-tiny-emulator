
pub struct GeneralRegisters {
    pub esp: u32, pub ebp: u32,
    pub eax: u32, pub ebx: u32, pub ecx: u32, pub edx: u32, pub edi:u32, pub esi:u32, 
    pub e8: u32, pub e9: u32, pub e10: u32, pub e11: u32, pub e12: u32, pub e13: u32, pub e14: u32, pub e15: u32
}

impl GeneralRegisters {
    pub fn new() -> GeneralRegisters {
        GeneralRegisters{
            esp: 0x7c00, ebp: 0,
            eax: 0, ebx: 0, ecx:0, edx:0, edi:0, esi:0,
            e8: 0, e9: 0, e10: 0, e11: 0, e12: 0, e13: 0, e14: 0, e15: 0
        }
    }

    pub fn print_regs(&self) {
        println!{"------Registers------"}
        println!("esp: 0x{:x}", self.esp);
        println!("ebp: 0x{:x}", self.ebp);
        println!("eax: 0x{:x}", self.eax);
        println!("ebx: 0x{:x}", self.ebx);
        println!("ecx: 0x{:x}", self.ecx);
        println!("edx: 0x{:x}", self.edx);
        println!("edi: 0x{:x}", self.edi);
        println!("esi: 0x{:x}", self.esi);
        println!("e8 : 0x{:x}", self.e8);
        println!("e9 : 0x{:x}", self.e9);
        println!("e10: 0x{:x}", self.e10);
        println!("e11: 0x{:x}", self.e11);
        println!("e12: 0x{:x}", self.e12);
        println!("e13: 0x{:x}", self.e13);
        println!("e14: 0x{:x}", self.e14);
        println!("e15: 0x{:x}", self.e15);
        println!{"--------End----------"}
    }
}