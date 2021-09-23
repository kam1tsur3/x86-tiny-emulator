pub const EAX_INDEX :u32 = 0;
pub const EBX_INDEX :u32 = 1;
pub const ECX_INDEX :u32 = 2;
pub const EDX_INDEX :u32 = 3;
pub const ESI_INDEX :u32 = 4;
pub const EDI_INDEX :u32 = 5;
pub const EBP_INDEX :u32 = 6;
pub const ESP_INDEX :u32 = 7;


pub struct GeneralRegisters {
    pub eax: u32, pub ebx: u32, pub ecx: u32, pub edx: u32, pub esi:u32, pub edi:u32, 
    pub ebp: u32, pub esp: u32,
}

impl GeneralRegisters {
    pub fn new() -> GeneralRegisters {
        GeneralRegisters{
            eax: 0, ebx: 0, ecx:0, edx:0, esi:0, edi:0,
            ebp: 0, esp: 0x7c00
        }
    }

    pub fn print_regs(&self) {
        println!{"------Registers------"}
        println!("eax: 0x{:x}", self.eax);
        println!("ebx: 0x{:x}", self.ebx);
        println!("ecx: 0x{:x}", self.ecx);
        println!("edx: 0x{:x}", self.edx);
        println!("esi: 0x{:x}", self.esi);
        println!("edi: 0x{:x}", self.edi);
        println!("ebp: 0x{:x}", self.ebp);
        println!("esp: 0x{:x}", self.esp);
        println!{"--------End----------"}
    }
}