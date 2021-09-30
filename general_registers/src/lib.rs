// 32bit registers
pub const EAX_INDEX :u8 = 0;
pub const ECX_INDEX :u8 = 1;
pub const EDX_INDEX :u8 = 2;
pub const EBX_INDEX :u8 = 3;
pub const ESP_INDEX :u8 = 4;
pub const EBP_INDEX :u8 = 5;
pub const ESI_INDEX :u8 = 6;
pub const EDI_INDEX :u8 = 7;
// 16bit registers
pub const AX_INDEX :u8 = 0;
pub const CX_INDEX :u8 = 1;
pub const DX_INDEX :u8 = 2;
pub const BX_INDEX :u8 = 3;
pub const SP_INDEX :u8 = 4;
pub const BP_INDEX :u8 = 5;
pub const SI_INDEX :u8 = 6;
pub const DI_INDEX :u8 = 7;
// 8bit registers
pub const AL_INDEX :u8 = 0;
pub const CL_INDEX :u8 = 1;
pub const DL_INDEX :u8 = 2;
pub const BL_INDEX :u8 = 3;
pub const AH_INDEX :u8 = 4;
pub const CH_INDEX :u8 = 5;
pub const DH_INDEX :u8 = 6;
pub const BH_INDEX :u8 = 7;

// eflags indexes
pub const CARRY_INDEX: u8 = 0;
pub const ZERO_INDEX: u8 = 6;
pub const SIGN_INDEX: u8 = 7;
pub const OVERFLOW_INDEX: u8 = 11;
pub struct GeneralRegisters {
    pub eax: u32, pub ecx: u32, pub edx: u32, pub ebx: u32,  
    pub esp: u32, pub ebp: u32,
    pub esi: u32, pub edi: u32,
}

impl GeneralRegisters {
    pub fn new() -> GeneralRegisters {
        GeneralRegisters{
            eax: 0, ecx:0, edx:0, ebx: 0, 
            esp: 0x7c00, ebp: 0, 
            esi: 0, edi:0,
        }
    }

    pub fn print_regs(&self) {
        println!{"------Registers------"}
        println!("eax: 0x{:x}", self.eax);
        println!("ecx: 0x{:x}", self.ecx);
        println!("edx: 0x{:x}", self.edx);
        println!("ebx: 0x{:x}", self.ebx);
        println!("esp: 0x{:x}", self.esp);
        println!("ebp: 0x{:x}", self.ebp);
        println!("esi: 0x{:x}", self.esi);
        println!("edi: 0x{:x}", self.edi);
        println!{"--------End----------"}
    }
}