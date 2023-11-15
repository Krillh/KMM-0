use std::collections::HashMap;

union split16 {
    u: u16,
    b: (u8, u8),
}
union split32 {
    u: u32,
    b: (u8, u8, u8, u8),
}

//#[derive(Error)]
pub enum AssemblyError {
    File(std::io::Error),
    UndefinedVariable(String, usize),
}

enum Token {}
enum Expr {
    Const(VarType),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    TypeCast(VarType),
}
impl Expr {
    fn get_type(&self) -> Result<VarType, AssemblyError> {
        todo!();
        // match self {
        // 	Expr::Const(t) => Ok(t.clone()),
        // 	Expr::Add(a, b) => {
        // 		let a = a.get_type()?;
        // 		let b = b.get_type()?;
        // 		if a == b {return a}
        // 		return Err(AssemblyError::File);
        // 	},
        // 	_ => panic!("expr get_type not finished!"),
        // }
    }
}
enum VarType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    SlimPtr,           // [ptr:4]
    Ptr(Box<VarType>), // [ptr:4][type:4]
    Struct(Vec<VarType>),
    Array(Box<VarType>, usize),
    Tuple(Vec<VarType>),
    Func(Vec<VarType>, Box<VarType>),
}
impl VarType {
    fn size(&self) -> u32 {
        match self {
            VarType::U8 => 1,
            VarType::U16 => 2,
            VarType::U32 => 4,
            VarType::U64 => 8,
            VarType::I8 => 1,
            VarType::I16 => 2,
            VarType::I32 => 4,
            VarType::I64 => 8,
            VarType::SlimPtr => 4,
            VarType::Ptr(_) => 8,
            VarType::Struct(n) => n.iter().map(|t| t.size()).sum::<u32>(),
            VarType::Array(t, s) => t.size() * (*s as u32),
            VarType::Tuple(n) => n.iter().map(|t| t.size()).sum::<u32>(),
            VarType::Func(_, _) => 4,
        }
    }
}
enum VarStore {
    Reg,
    ZeroPage,
    Stack,
}
struct Scope {
    name: String,
    code: Vec<Token>,
    vars: HashMap<String, VarType, VarStore>,
}
pub enum Instruction {
    Nop,

    JmpConst24(u32),
    JmpConst32(u32),
    JmpOffsetA8(u8),
    JmpOffsetS8(u8),
    JmpOffsetA16(u16),
    JmpOffsetS16(u16),
    JmpOffsetA32(u32),
    JmpOffsetS32(u32),

    JCConst24(u32),
    JCConst32(u32),
    JCOffsetA8(u8),
    JCOffsetS8(u8),
    JCOffsetA16(u16),
    JCOffsetS16(u16),
    JCOffsetA32(u32),
    JCOffsetS32(u32),

    JNCConst24(u32),
    JNCConst32(u32),
    JNCOffsetA8(u8),
    JNCOffsetS8(u8),
    JNCOffsetA16(u16),
    JNCOffsetS16(u16),
    JNCOffsetA32(u32),
    JNCOffsetS32(u32),

    JZConst24(u32),
    JZConst32(u32),
    JZOffsetA8(u8),
    JZOffsetS8(u8),
    JZOffsetA16(u16),
    JZOffsetS16(u16),
    JZOffsetA32(u32),
    JZOffsetS32(u32),

    JNZConst24(u32),
    JNZConst32(u32),
    JNZOffsetA8(u8),
    JNZOffsetS8(u8),
    JNZOffsetA16(u16),
    JNZOffsetS16(u16),
    JNZOffsetA32(u32),
    JNZOffsetS32(u32),

    AddRRR(u8, u8, u8),
    AddZRR(u8, u8, u8),
    IncR1(u8),
    AddRRZ(u8, u8, u8),
    AddRZZ(u8, u8, u8),
    AddZZZ(u8, u8, u8),

    AddcRRR(u8, u8, u8),
    AddcZRR(u8, u8, u8),
    IncR4(u8),
    AddcRRZ(u8, u8, u8),
    AddcRZZ(u8, u8, u8),
    AddcZZZ(u8, u8, u8),

    SubRRR(u8, u8, u8),
    SubZRR(u8, u8, u8),
    SubRZR(u8, u8, u8),
    SubRRZ(u8, u8, u8),
    SubRZZ(u8, u8, u8),
    SubZZZ(u8, u8, u8),

    SubbRRR(u8, u8, u8),
    SubbZRR(u8, u8, u8),
    SubbRZR(u8, u8, u8),
    SubbRRZ(u8, u8, u8),
    SubbRZZ(u8, u8, u8),
    SubbZZZ(u8, u8, u8),

    MulRRR(u8, u8, u8),
    MulZRR(u8, u8, u8),
    MulRRZ(u8, u8, u8),
    MulRZZ(u8, u8, u8),
    MulZZZ(u8, u8, u8),

    DivRRR(u8, u8, u8),
    DivZRR(u8, u8, u8),
    DivRZR(u8, u8, u8),
    DivRRZ(u8, u8, u8),
    DivRZZ(u8, u8, u8),
    DivZZZ(u8, u8, u8),

    AndRRR(u8, u8, u8),
    AndZRR(u8, u8, u8),
    AndRRZ(u8, u8, u8),
    AndRZZ(u8, u8, u8),
    AndZZZ(u8, u8, u8),

    OrRRR(u8, u8, u8),
    OrZRR(u8, u8, u8),
    OrRRZ(u8, u8, u8),
    OrRZZ(u8, u8, u8),
    OrZZZ(u8, u8, u8),

    XorRRR(u8, u8, u8),
    XorZRR(u8, u8, u8),
    XorRRZ(u8, u8, u8),
    XorRZZ(u8, u8, u8),
    XorZZZ(u8, u8, u8),

    NotRR(u8, u8),
    NotZR(u8, u8),
    NotRZ(u8, u8),
    NotZZ(u8, u8),

    DecR1(u8),
    DecR4(u8),

    RemRRR(u8, u8, u8),
    RemZRR(u8, u8, u8),
    RemRZR(u8, u8, u8),
    RemRRZ(u8, u8, u8),
    RemRZZ(u8, u8, u8),
    RemZZZ(u8, u8, u8),

    ShlRRR(u8, u8, u8),
    ShlZRR(u8, u8, u8),
    ShlRCR(u8, u8, u8),
    ShlZCR(u8, u8, u8),
    ShlRCZ(u8, u8, u8),
    ShlZCZ(u8, u8, u8),

    ShrRRR(u8, u8, u8),
    ShrZRR(u8, u8, u8),
    ShrRCR(u8, u8, u8),
    ShrZCR(u8, u8, u8),
    ShrRCZ(u8, u8, u8),
    ShrZCZ(u8, u8, u8),

    PrintChar_R(u8),
    PrintChar_C8(u8),

    Read8_Cptr_R(u32, u8),
    Read16_Cptr_R(u32, u8),
    Read32_Cptr_R(u32, u8),
    Read8_Rptr_R(u8, u8),
    Read16_Rptr_R(u8, u8),
    Read32_Rptr_R(u8, u8),
    Write8_R_Cptr(u8, u32),
    Write16_R_Cptr(u8, u32),
    Write32_R_Cptr(u8, u32),
    Write8_R_Rptr(u8, u8),
    Write16_R_Rptr(u8, u8),
    Write32_R_Rptr(u8, u8),
    Write8_C8_Cptr(u8, u32),
    Write8_C8_Rptr(u16, u8),
    Write16_C16_Cptr(u16, u32),
    Write16_C16_Rptr(u16, u8),
    Write32_C32_Cptr(u32, u32),
    Write32_C32_Rptr(u32, u8),

    MovRR(u8, u8),
    MovR4R4(u8, u8, u8),
    Mov4R4R(u8, u8, u8),
    Mov44(u8),
    ConstRegU8(u8, u8),
    ConstRegU16(u8, u16),
    ConstRegU32(u8, u32),

    SetZF,
    ClrZF,
    SetCF,
    ClrCF,
    IntEnable,
    IntDisable,

    ExplicitHaltAndExit,
    DebugPrintReg(u8),
}

pub trait Assembler {
    fn assemble_file(path: &str) -> Result<Vec<u8>, AssemblyError>;
}

pub struct Ver0;
impl Ver0 {
    fn tokenize(code: &str) -> Result<Scope, AssemblyError> {
        todo!();
    }
    fn flatten_tokens(code: Scope) -> Result<Vec<Instruction>, AssemblyError> {
        todo!();
    }
    pub fn assemble_to_bytes(code: Vec<Instruction>) -> Vec<u8> {
        let mut bytes = vec![];

        fn b0_16(n: u16) -> u8 {
            unsafe { split16 { u: n }.b.0 }
        }
        fn b1_16(n: u16) -> u8 {
            unsafe { split16 { u: n }.b.1 }
        }
        fn b0_24(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.0 }
        }
        fn b1_24(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.1 }
        }
        fn b2_24(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.2 }
        }
        fn b0_32(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.0 }
        }
        fn b1_32(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.1 }
        }
        fn b2_32(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.2 }
        }
        fn b3_32(n: u32) -> u8 {
            unsafe { split32 { u: n }.b.3 }
        }
        // fn s_16(v: u16) -> &'static [u8] {
        // 	let s = split16{u:v}.b;
        // 	return &[s.0,s.1]
        // }
        // fn s_24(v: u32) -> &'static [u8] {
        // 	let s = split32{u:v}.b;
        // 	return &[s.0,s.1,s.2]
        // }
        // fn s_32(v: u32) -> &'static [u8] {
        // 	let s = split32{u:v}.b;
        // 	return &[s.0,s.1,s.2,s.3]
        // }

        //TODO FIX BYTE APPENDING
        // ! BYTE APPENDING IS COMPLETELY BROKEN RIGHT NOW
        // ! CURRENTLY DOES NOT COMPILE !!!!!!!------------------------------------------

        type I = Instruction;
        for i in code {
            match i {
                I::Nop => bytes.extend([0x00]),
                I::JmpConst24(dest) => {
                    bytes.extend([0x02, 0x00, b0_24(dest), b1_24(dest), b2_24(dest)])
                }
                I::JmpConst32(dest) => {
                    bytes.extend([0x02, 0x01, b0_24(dest), b1_24(dest), b2_24(dest)])
                }
                I::JmpOffsetA8(ofs) => bytes.extend([0x02, 0x02, ofs]),
                I::JmpOffsetS8(ofs) => bytes.extend([0x02, 0x03, ofs]),
                I::JmpOffsetA16(ofs) => bytes.extend([0x02, 0x04, b0_16(ofs), b1_16(ofs)]),
                I::JmpOffsetS16(ofs) => bytes.extend([0x02, 0x05, b0_16(ofs), b1_16(ofs)]),
                I::JmpOffsetA32(ofs) => {
                    bytes.extend([0x02, 0x06, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JmpOffsetS32(ofs) => {
                    bytes.extend([0x02, 0x07, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JCConst24(dest) => {
                    bytes.extend([0x03, 0x00, b0_24(dest), b1_24(dest), b2_24(dest)])
                }
                I::JCConst32(dest) => bytes.extend([
                    0x03,
                    0x01,
                    b0_32(dest),
                    b1_32(dest),
                    b2_32(dest),
                    b3_32(dest),
                ]),
                I::JCOffsetA8(ofs) => bytes.extend([0x03, 0x02, ofs]),
                I::JCOffsetS8(ofs) => bytes.extend([0x03, 0x03, ofs]),
                I::JCOffsetA16(ofs) => bytes.extend([0x03, 0x04, b0_16(ofs), b1_16(ofs)]),
                I::JCOffsetS16(ofs) => bytes.extend([0x03, 0x05, b0_16(ofs), b1_16(ofs)]),
                I::JCOffsetA32(ofs) => {
                    bytes.extend([0x03, 0x06, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JCOffsetS32(ofs) => {
                    bytes.extend([0x03, 0x07, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JNCConst24(dest) => {
                    bytes.extend([0x03, 0x08, b0_24(dest), b1_24(dest), b2_24(dest)])
                }
                I::JNCConst32(dest) => bytes.extend([
                    0x03,
                    0x09,
                    b0_32(dest),
                    b1_32(dest),
                    b2_32(dest),
                    b3_32(dest),
                ]),
                I::JNCOffsetA8(ofs) => bytes.extend([0x03, 0x0a, ofs]),
                I::JNCOffsetS8(ofs) => bytes.extend([0x03, 0x0b, ofs]),
                I::JNCOffsetA16(ofs) => bytes.extend([0x03, 0x0c, b0_16(ofs), b1_16(ofs)]),
                I::JNCOffsetS16(ofs) => bytes.extend([0x03, 0x0d, b0_16(ofs), b1_16(ofs)]),
                I::JNCOffsetA32(ofs) => {
                    bytes.extend([0x03, 0x0e, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JNCOffsetS32(ofs) => {
                    bytes.extend([0x03, 0x0f, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JZConst24(dest) => {
                    bytes.extend([0x03, 0x10, b0_24(dest), b1_24(dest), b2_24(dest)])
                }
                I::JZConst32(dest) => bytes.extend([
                    0x03,
                    0x11,
                    b0_32(dest),
                    b1_32(dest),
                    b2_32(dest),
                    b3_32(dest),
                ]),
                I::JZOffsetA8(ofs) => bytes.extend([0x03, 0x12, ofs]),
                I::JZOffsetS8(ofs) => bytes.extend([0x03, 0x13, ofs]),
                I::JZOffsetA16(ofs) => bytes.extend([0x03, 0x14, b0_16(ofs), b1_16(ofs)]),
                I::JZOffsetS16(ofs) => bytes.extend([0x03, 0x15, b0_16(ofs), b1_16(ofs)]),
                I::JZOffsetA32(ofs) => {
                    bytes.extend([0x03, 0x16, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JZOffsetS32(ofs) => {
                    bytes.extend([0x03, 0x17, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JNZConst24(dest) => {
                    bytes.extend([0x03, 0x18, b0_24(dest), b1_24(dest), b2_24(dest)])
                }
                I::JNZConst32(dest) => bytes.extend([
                    0x03,
                    0x19,
                    b0_32(dest),
                    b1_32(dest),
                    b2_32(dest),
                    b3_32(dest),
                ]),
                I::JNZOffsetA8(ofs) => bytes.extend([0x03, 0x1a, ofs]),
                I::JNZOffsetS8(ofs) => bytes.extend([0x03, 0x1b, ofs]),
                I::JNZOffsetA16(ofs) => bytes.extend([0x03, 0x1c, b0_16(ofs), b1_16(ofs)]),
                I::JNZOffsetS16(ofs) => bytes.extend([0x03, 0x1d, b0_16(ofs), b1_16(ofs)]),
                I::JNZOffsetA32(ofs) => {
                    bytes.extend([0x03, 0x1e, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }
                I::JNZOffsetS32(ofs) => {
                    bytes.extend([0x03, 0x1f, b0_32(ofs), b1_32(ofs), b2_32(ofs), b3_32(ofs)])
                }

                I::AddRRR(a, b, d) => bytes.extend([0x04, 0x00, a, b, d]),
                I::AddZRR(a, b, d) => bytes.extend([0x04, 0x01, a, b, d]),
                I::IncR1(r) => bytes.extend([0x04, 0x02, r]),
                I::AddRRZ(a, b, d) => bytes.extend([0x04, 0x03, a, b, d]),
                I::AddRZZ(a, b, d) => bytes.extend([0x04, 0x04, a, b, d]),
                I::AddZZZ(a, b, d) => bytes.extend([0x04, 0x05, a, b, d]),
                I::AddcRRR(a, b, d) => bytes.extend([0x04, 0x06, a, b, d]),
                I::AddcZRR(a, b, d) => bytes.extend([0x04, 0x07, a, b, d]),
                I::IncR4(r) => bytes.extend([0x04, 0x08, r]),
                I::AddcRRZ(a, b, d) => bytes.extend([0x04, 0x09, a, b, d]),
                I::AddcRZZ(a, b, d) => bytes.extend([0x04, 0x0a, a, b, d]),
                I::AddcZZZ(a, b, d) => bytes.extend([0x04, 0x0b, a, b, d]),
                I::SubRRR(a, b, d) => bytes.extend([0x04, 0x0c, a, b, d]),
                I::SubZRR(a, b, d) => bytes.extend([0x04, 0x0d, a, b, d]),
                I::SubRZR(a, b, d) => bytes.extend([0x04, 0x0e, a, b, d]),
                I::SubRRZ(a, b, d) => bytes.extend([0x04, 0x0f, a, b, d]),
                I::SubRZZ(a, b, d) => bytes.extend([0x04, 0x10, a, b, d]),
                I::SubZZZ(a, b, d) => bytes.extend([0x04, 0x11, a, b, d]),
                I::SubbRRR(a, b, d) => bytes.extend([0x04, 0x12, a, b, d]),
                I::SubbZRR(a, b, d) => bytes.extend([0x04, 0x13, a, b, d]),
                I::SubbRZR(a, b, d) => bytes.extend([0x04, 0x14, a, b, d]),
                I::SubbRRZ(a, b, d) => bytes.extend([0x04, 0x15, a, b, d]),
                I::SubbRZZ(a, b, d) => bytes.extend([0x04, 0x16, a, b, d]),
                I::SubbZZZ(a, b, d) => bytes.extend([0x04, 0x17, a, b, d]),
                I::MulRRR(a, b, d) => bytes.extend([0x04, 0x18, a, b, d]),
                I::MulZRR(a, b, d) => bytes.extend([0x04, 0x19, a, b, d]),
                I::MulRRZ(a, b, d) => bytes.extend([0x04, 0x1a, a, b, d]),
                I::MulRZZ(a, b, d) => bytes.extend([0x04, 0x1b, a, b, d]),
                I::MulZZZ(a, b, d) => bytes.extend([0x04, 0x1c, a, b, d]),
                I::DivRRR(a, b, d) => bytes.extend([0x04, 0x1d, a, b, d]),
                I::DivZRR(a, b, d) => bytes.extend([0x04, 0x1e, a, b, d]),
                I::DivRZR(a, b, d) => bytes.extend([0x04, 0x1f, a, b, d]),
                I::DivRRZ(a, b, d) => bytes.extend([0x04, 0x20, a, b, d]),
                I::DivRZZ(a, b, d) => bytes.extend([0x04, 0x21, a, b, d]),
                I::DivZZZ(a, b, d) => bytes.extend([0x04, 0x22, a, b, d]),
                I::AndRRR(a, b, d) => bytes.extend([0x04, 0x23, a, b, d]),
                I::AndZRR(a, b, d) => bytes.extend([0x04, 0x24, a, b, d]),
                I::AndRRZ(a, b, d) => bytes.extend([0x04, 0x25, a, b, d]),
                I::AndRZZ(a, b, d) => bytes.extend([0x04, 0x26, a, b, d]),
                I::AndZZZ(a, b, d) => bytes.extend([0x04, 0x27, a, b, d]),
                I::OrRRR(a, b, d) => bytes.extend([0x04, 0x28, a, b, d]),
                I::OrZRR(a, b, d) => bytes.extend([0x04, 0x29, a, b, d]),
                I::OrRRZ(a, b, d) => bytes.extend([0x04, 0x2a, a, b, d]),
                I::OrRZZ(a, b, d) => bytes.extend([0x04, 0x2b, a, b, d]),
                I::OrZZZ(a, b, d) => bytes.extend([0x04, 0x2c, a, b, d]),
                I::XorRRR(a, b, d) => bytes.extend([0x04, 0x2d, a, b, d]),
                I::XorZRR(a, b, d) => bytes.extend([0x04, 0x2e, a, b, d]),
                I::XorRRZ(a, b, d) => bytes.extend([0x04, 0x2f, a, b, d]),
                I::XorRZZ(a, b, d) => bytes.extend([0x04, 0x30, a, b, d]),
                I::XorZZZ(a, b, d) => bytes.extend([0x04, 0x31, a, b, d]),
                I::NotRR(a, d) => bytes.extend([0x04, 0x32, a, d]),
                I::NotZR(a, d) => bytes.extend([0x04, 0x33, a, d]),
                I::NotRZ(a, d) => bytes.extend([0x04, 0x34, a, d]),
                I::NotZZ(a, d) => bytes.extend([0x04, 0x35, a, d]),
                I::DecR1(r) => bytes.extend([0x04, 0x36, r]),
                I::DecR4(r) => bytes.extend([0x04, 0x37, r]),
                I::RemRRR(a, b, d) => bytes.extend([0x04, 0x38, a, b, d]),
                I::RemZRR(a, b, d) => bytes.extend([0x04, 0x39, a, b, d]),
                I::RemRZR(a, b, d) => bytes.extend([0x04, 0x3a, a, b, d]),
                I::RemRRZ(a, b, d) => bytes.extend([0x04, 0x3b, a, b, d]),
                I::RemRZZ(a, b, d) => bytes.extend([0x04, 0x3c, a, b, d]),
                I::RemZZZ(a, b, d) => bytes.extend([0x04, 0x3d, a, b, d]),
                I::ShlRRR(a, b, d) => bytes.extend([0x04, 0x3e, a, b, d]),
                I::ShlZRR(a, b, d) => bytes.extend([0x04, 0x3f, a, b, d]),
                I::ShlRCR(a, b, d) => bytes.extend([0x04, 0x40, a, b, d]),
                I::ShlZCR(a, b, d) => bytes.extend([0x04, 0x41, a, b, d]),
                I::ShlRCZ(a, b, d) => bytes.extend([0x04, 0x42, a, b, d]),
                I::ShlZCZ(a, b, d) => bytes.extend([0x04, 0x43, a, b, d]),
                I::ShrRRR(a, b, d) => bytes.extend([0x04, 0x44, a, b, d]),
                I::ShrZRR(a, b, d) => bytes.extend([0x04, 0x45, a, b, d]),
                I::ShrRCR(a, b, d) => bytes.extend([0x04, 0x46, a, b, d]),
                I::ShrZCR(a, b, d) => bytes.extend([0x04, 0x47, a, b, d]),
                I::ShrRCZ(a, b, d) => bytes.extend([0x04, 0x48, a, b, d]),
                I::ShrZCZ(a, b, d) => bytes.extend([0x04, 0x49, a, b, d]),
                I::PrintChar_R(r) => bytes.extend([0x06, 0x00, r]),
                I::PrintChar_C8(c) => bytes.extend([0x06, 0x00, c]),
                I::Read8_Cptr_R(ptr, r) => bytes.extend([
                    0x07,
                    0x00,
                    b0_32(ptr),
                    b1_32(ptr),
                    b2_32(ptr),
                    b3_32(ptr),
                    r,
                ]),
                I::Read16_Cptr_R(_, _) => todo!(),
                I::Read32_Cptr_R(_, _) => todo!(),
                I::Read8_Rptr_R(_, _) => todo!(),
                I::Read16_Rptr_R(_, _) => todo!(),
                I::Read32_Rptr_R(_, _) => todo!(),
                I::Write8_R_Cptr(_, _) => todo!(),
                I::Write16_R_Cptr(_, _) => todo!(),
                I::Write32_R_Cptr(_, _) => todo!(),
                I::Write8_R_Rptr(_, _) => todo!(),
                I::Write16_R_Rptr(_, _) => todo!(),
                I::Write32_R_Rptr(_, _) => todo!(),
                I::Write8_C8_Cptr(_, _) => todo!(),
                I::Write8_C8_Rptr(_, _) => todo!(),
                I::Write16_C16_Cptr(_, _) => todo!(),
                I::Write16_C16_Rptr(_, _) => todo!(),
                I::Write32_C32_Cptr(_, _) => todo!(),
                I::Write32_C32_Rptr(_, _) => todo!(),
                I::MovRR(_, _) => todo!(),
                I::MovR4R4(_, _, _) => todo!(),
                I::Mov4R4R(_, _, _) => todo!(),
                I::Mov44(_) => todo!(),
                I::ConstRegU8(r, c) => bytes.extend([0x14, c, r]),
                I::ConstRegU16(r, c) => bytes.extend([0x15, b0_16(c), b1_16(c), r]),
                I::ConstRegU32(r, c) => {
                    bytes.extend([0x16, b0_32(c), b1_32(c), b2_32(c), b3_32(c), r])
                }
                I::SetZF => bytes.extend([0x20]),
                I::ClrZF => bytes.extend([0x21]),
                I::SetCF => bytes.extend([0x22]),
                I::ClrCF => bytes.extend([0x23]),
                I::IntEnable => bytes.extend([0x24]),
                I::IntDisable => bytes.extend([0x25]),
                I::ExplicitHaltAndExit => bytes.extend([0xFE]),
                I::DebugPrintReg(r) => bytes.extend([0xFF, 0x01, r]),
                //_ => panic!(),
            }
        }
        return bytes;
    }
}
impl Assembler for Ver0 {
    fn assemble_file(path: &str) -> Result<Vec<u8>, AssemblyError> {
        return Ok(vec![0]);
    }
}
