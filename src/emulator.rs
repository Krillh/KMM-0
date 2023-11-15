use std::io::{self, Write};

union BitConvert32 {
    u: u32,
    b: (u8, u8, u8, u8),
}

fn u32_split(n: u32) -> (u8, u8, u8, u8) {
    unsafe { BitConvert32 { u: n }.b }
}

fn u32_join(n: (u8, u8, u8, u8)) -> u32 {
    unsafe { BitConvert32 { b: n }.u }
}

// register constants
const IP: usize = 0x8; // instruction pointer
const SP: usize = 0x9; // stack pointer
const RP: usize = 0xa; // interrupt return pointer
const TR: usize = 0xb; // temp register
const IX: usize = 0xc; // index register

// fault constants
const EXPLICIT_HALT_AND_EXIT: u8 = 0x01;
const INVALID_INSTRUCTION: u8 = 0x10;
const INVALID_INSTRUCTION_VARIANT: u8 = 0x11;
const INVALID_CHAR: u8 = 0x20;

pub struct KMM0 {
    print_exec: bool,
    debug_clock_speed_hz: f64,
    debug_uptime_cycles: u64,

    acc: u32,
    z: bool,
    c: bool,
    int_enable: bool,
    int_idx: u8,
    fault: u8,
    reg: [u32; 16],
    i_mem: Vec<u8>,
    d_mem: Vec<u8>,
}
impl KMM0 {
    pub fn new() -> Self {
        KMM0 {
            print_exec: false,
            debug_clock_speed_hz: 1f64,
            debug_uptime_cycles: 0u64,

            acc: 0u32,
            z: false,
            c: false,
            int_enable: false,
            int_idx: 0u8,
            fault: 0u8,
            reg: [0u32; 16],
            i_mem: Vec::new(),
            d_mem: Vec::new(),
        }
    }
    pub fn print_exec(mut self) -> Self {
        self.print_exec = true;
        return self;
    }
    pub fn init_mem(mut self, size: usize) -> Self {
        self.d_mem = vec![0x00; size];
        return self;
    }
    pub fn load_code(mut self, code: Vec<u8>) -> Self {
        self.i_mem = code.clone();
        self.reg[IP] = 0x100;
        return self;
    }
    pub fn clock_speed_hz(mut self, hz: f64) -> Self {
        self.debug_clock_speed_hz = hz;
        return self;
    }
    pub fn clock(&mut self) {
        if self.debug_clock_speed_hz != 0. {
            std::thread::sleep(std::time::Duration::from_secs_f64(
                1. / self.debug_clock_speed_hz,
            ));
        }
        self.debug_uptime_cycles += 1;
    }
    pub fn ix_reg(&self) -> usize {
        self.reg[IX] as usize
    }
    pub fn ip_reg(&self) -> usize {
        self.reg[IP] as usize
    }
    pub fn sp_reg(&self) -> usize {
        self.reg[SP] as usize
    }
    pub fn mrn8_ip(&mut self) -> u8 {
        self.clock();
        let n = self.i_mem[self.ip_reg()];
        self.reg[IP] += 1;
        if self.print_exec {
            print!("{:02x?} ", n);
            io::stdout().flush().unwrap();
        }
        return n;
    }
    pub fn mrn16_ip(&mut self) -> u16 {
        u32_join((0, 0, self.mrn8_ip(), self.mrn8_ip())) as u16
    }
    pub fn mrn24_ip(&mut self) -> u32 {
        u32_join((0, self.mrn8_ip(), self.mrn8_ip(), self.mrn8_ip()))
    }
    pub fn mrn32_ip(&mut self) -> u32 {
        u32_join((
            self.mrn8_ip(),
            self.mrn8_ip(),
            self.mrn8_ip(),
            self.mrn8_ip(),
        ))
    }
    pub fn mrn8_sp(&mut self) -> u8 {
        self.clock();
        let n = self.d_mem[self.sp_reg()];
        self.reg[SP] -= 1;
        return n;
    }
    pub fn mrn16_sp(&mut self) -> u16 {
        u32_join((0, 0, self.mrn8_sp(), self.mrn8_sp())) as u16
    }
    pub fn mrn32_sp(&mut self) -> u32 {
        u32_join((
            self.mrn8_sp(),
            self.mrn8_sp(),
            self.mrn8_sp(),
            self.mrn8_sp(),
        ))
    }
    pub fn zpr(&mut self, ix: u8) -> u32 {
        self.clock();
        self.clock();
        let ix = (ix as usize) * 4;
        u32_join((
            self.d_mem[ix + 0],
            self.d_mem[ix + 1],
            self.d_mem[ix + 2],
            self.d_mem[ix + 3],
        ))
    }
    pub fn zpw(&mut self, v: u32, ix: u8) {
        self.clock();
        self.clock();
        let ix = (ix as usize) * 4;
        let b = u32_split(v);
        self.d_mem[ix + 0] = b.0;
        self.d_mem[ix + 1] = b.1;
        self.d_mem[ix + 2] = b.2;
        self.d_mem[ix + 3] = b.3;
    }
    pub fn mwn8_sp(&mut self) {
        self.clock();
        let ix = self.sp_reg();
        self.d_mem[ix] = u32_split(self.acc).0;
        self.reg[SP] += 1;
    }
    pub fn fault(&mut self, fault: u8) {
        self.fault = fault;
        self.int_enable = true;
        self.int_idx = 0;
    }
    pub fn execute(&mut self) {
        macro_rules! flush_debug {
            () => {
                if self.print_exec {
                    println!("");
                }
            };
        }

        macro_rules! set_flags {
            ($n:ident, $o:ident) => {
                self.c = $o;
                self.z = $n == 0;
            };
        }

        macro_rules! do_math {
			(rrr $op:ident$( $c:ident)?) => {
				let a = self.reg[self.mrn8_ip() as usize];
				let b = self.reg[self.mrn8_ip() as usize];
				let d = self.mrn8_ip() as usize;
				let (n, o) = a.$op(b,$(self.$c)?);
				self.reg[d] = n;
				set_flags!(n, o);
			};
			(zrr $op:ident $($c:ident)?) => {
				let zpix_a = self.mrn8_ip();
				let a = self.zpr(zpix_a);
				let b = self.reg[self.mrn8_ip() as usize];
				let d = self.mrn8_ip() as usize;
				let (n, o) = a.$op(b,$(self.$c)?);
				self.reg[d] = n;
				set_flags!(n, o);
			};
			(rzr $op:ident $($c:ident)?) => {
				let a = self.reg[self.mrn8_ip() as usize];
				let zpix_b = self.mrn8_ip();
				let b = self.zpr(zpix_b);
				let d = self.mrn8_ip() as usize;
				let (n, o) = a.$op(b,$(self.$c)?);
				self.reg[d] = n;
				set_flags!(n, o);
			};
			(rrz $op:ident $($c:ident)?) => {
				let a = self.reg[self.mrn8_ip() as usize];
				let b = self.reg[self.mrn8_ip() as usize];
				let d = self.mrn8_ip();
				let (n, o) = a.$op(b,$(self.$c)?);
				self.zpw(n, d);
				set_flags!(n, o);
			};
			(rzz $op:ident $($c:ident)?) => {
				let a = self.reg[self.mrn8_ip() as usize];
				let zpix_b = self.mrn8_ip();
				let b = self.zpr(zpix_b);
				let d = self.mrn8_ip();
				let (n, o) = a.$op(b,$(self.$c)?);
				self.zpw(n, d);
				set_flags!(n, o);
			};
			(zzz $op:ident $($c:ident)?) => {
				let zpix_a = self.mrn8_ip();
				let a = self.zpr(zpix_a);
				let zpix_b = self.mrn8_ip();
				let b = self.zpr(zpix_b);
				let d = self.mrn8_ip();
				let (n, o) = a.$op(b,$(self.$c)?);
				self.zpw(n, d);
				set_flags!(n, o);
			};
		}

        // check for a fault
        if self.fault != 0 {
            println!(
                "\n\n-------------------------\nHARDWARE FAULT- CODE 0x{:02X?}",
                self.fault
            );
            panic!("hardware fault {:02X?}", self.fault);
        }

        // check for interrupts
        if self.int_enable & (self.int_idx != 0) {
            self.int_enable = false;
            self.reg[RP] = self.reg[IP];
            self.reg[IP] = 0x100 + (8 * (self.int_idx as u32));
        }

        // execute
        let i = self.mrn8_ip();
        match i {
            0x00 => {}
            0x02 => {
                // JUMP instructions
                let v = self.mrn8_ip();
                match &v {
                    0x00 => self.reg[IP] = self.mrn24_ip(), // JMP const_24
                    0x01 => self.reg[IP] = self.mrn32_ip(), // JMP const_32
                    0x02 => self.reg[IP] = self.reg[IP] + (self.mrn8_ip() as u32), // JMP_offset +const_8
                    0x03 => self.reg[IP] = self.reg[IP] - (self.mrn8_ip() as u32), // JMP_offset -const_8
                    0x04 => self.reg[IP] = self.reg[IP] + (self.mrn16_ip() as u32), // JMP_offset +const_16
                    0x05 => self.reg[IP] = self.reg[IP] - (self.mrn16_ip() as u32), // JMP_offset -const_16
                    0x06 => self.reg[IP] = self.reg[IP] + self.mrn32_ip(), // JMP_offset +const_32
                    0x07 => self.reg[IP] = self.reg[IP] - self.mrn32_ip(), // JMP_offset -const_32
                    _ => self.fault(INVALID_INSTRUCTION_VARIANT),
                }
            }
            0x03 => {
                // CONDITIONAL JUMP instructions
                let v = self.mrn8_ip();
                match &v {
                    0x00 => {
                        let j = self.mrn24_ip();
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC const_24
                    0x01 => {
                        let j = self.mrn32_ip();
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC const_32
                    0x02 => {
                        let j = self.reg[IP] + (self.mrn8_ip() as u32);
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC_offset +const_8
                    0x03 => {
                        let j = self.reg[IP] - (self.mrn8_ip() as u32);
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC_offset -const_8
                    0x04 => {
                        let j = self.reg[IP] + (self.mrn16_ip() as u32);
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC_offset +const_16
                    0x05 => {
                        let j = self.reg[IP] - (self.mrn16_ip() as u32);
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC_offset -const_16
                    0x06 => {
                        let j = self.reg[IP] + self.mrn32_ip();
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC_offset +const_32
                    0x07 => {
                        let j = self.reg[IP] - self.mrn32_ip();
                        if self.c {
                            self.reg[IP] = j
                        }
                    } // JC_offset -const_32

                    0x08 => {
                        let j = self.mrn24_ip();
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC const_24
                    0x09 => {
                        let j = self.mrn32_ip();
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC const_32
                    0x0a => {
                        let j = self.reg[IP] + (self.mrn8_ip() as u32);
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC_offset +const_8
                    0x0b => {
                        let j = self.reg[IP] - (self.mrn8_ip() as u32);
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC_offset -const_8
                    0x0c => {
                        let j = self.reg[IP] + (self.mrn16_ip() as u32);
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC_offset +const_16
                    0x0d => {
                        let j = self.reg[IP] - (self.mrn16_ip() as u32);
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC_offset -const_16
                    0x0e => {
                        let j = self.reg[IP] + self.mrn32_ip();
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC_offset +const_32
                    0x0f => {
                        let j = self.reg[IP] - self.mrn32_ip();
                        if !self.c {
                            self.reg[IP] = j
                        }
                    } // JNC_offset -const_32

                    0x10 => {
                        let j = self.mrn24_ip();
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ const_24
                    0x11 => {
                        let j = self.mrn32_ip();
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ const_32
                    0x12 => {
                        let j = self.reg[IP] + (self.mrn8_ip() as u32);
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ_offset +const_8
                    0x13 => {
                        let j = self.reg[IP] - (self.mrn8_ip() as u32);
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ_offset -const_8
                    0x14 => {
                        let j = self.reg[IP] + (self.mrn16_ip() as u32);
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ_offset +const_16
                    0x15 => {
                        let j = self.reg[IP] - (self.mrn16_ip() as u32);
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ_offset -const_16
                    0x16 => {
                        let j = self.reg[IP] + self.mrn32_ip();
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ_offset +const_32
                    0x17 => {
                        let j = self.reg[IP] - self.mrn32_ip();
                        if self.z {
                            self.reg[IP] = j
                        }
                    } // JZ_offset -const_32

                    0x18 => {
                        let j = self.mrn24_ip();
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ const_24
                    0x19 => {
                        let j = self.mrn32_ip();
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ const_32
                    0x1a => {
                        let j = self.reg[IP] + (self.mrn8_ip() as u32);
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ_offset +const_8
                    0x1b => {
                        let j = self.reg[IP] - (self.mrn8_ip() as u32);
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ_offset -const_8
                    0x1c => {
                        let j = self.reg[IP] + (self.mrn16_ip() as u32);
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ_offset +const_16
                    0x1d => {
                        let j = self.reg[IP] - (self.mrn16_ip() as u32);
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ_offset -const_16
                    0x1e => {
                        let j = self.reg[IP] + self.mrn32_ip();
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ_offset +const_32
                    0x1f => {
                        let j = self.reg[IP] - self.mrn32_ip();
                        if !self.z {
                            self.reg[IP] = j
                        }
                    } // JNZ_offset -const_32

                    _ => self.fault(INVALID_INSTRUCTION_VARIANT),
                }
            }
            0x04 => {
                // MATH instructions
                let v = self.mrn8_ip();
                match &v {
                    0x00 => {
                        // ADD reg + reg -> reg
                        do_math!(rrr overflowing_add);
                    }
                    0x01 => {
                        // ADD zpr + reg -> reg
                        do_math!(zrr overflowing_add);
                    }
                    0x02 => {
                        // INC reg += 1
                        let a = self.mrn8_ip() as usize;
                        let (n, o) = self.reg[a].overflowing_add(1);
                        self.reg[a] = n;
                        set_flags!(n, o);
                    }
                    0x03 => {
                        // ADD reg + reg -> zpr
                        do_math!(rrz overflowing_add);
                    }
                    0x04 => {
                        // ADD reg + zpr -> zpr
                        do_math!(rzz overflowing_add);
                    }
                    0x05 => {
                        // ADD zpr + zpr -> zpr
                        do_math!(zzz overflowing_add);
                    }

                    0x06 => {
                        // ADDc reg +c reg -> reg
                        do_math!(rrr carrying_add c);
                    }
                    0x07 => {
                        // ADDc zpr +c reg -> reg
                        do_math!(zrr carrying_add c);
                    }
                    0x08 => {
                        // INC reg += 4
                        let a = self.mrn8_ip() as usize;
                        let (n, o) = self.reg[a].overflowing_add(4);
                        self.reg[a] = n;
                        set_flags!(n, o);
                    }
                    0x09 => {
                        // ADDc reg +c reg -> zpr
                        do_math!(rrz carrying_add c);
                    }
                    0x0a => {
                        // ADDc reg +c zpr -> zpr
                        do_math!(rzz carrying_add c);
                    }
                    0x0b => {
                        // ADDc zpr +c zpr -> zpr
                        do_math!(zzz carrying_add c);
                    }
                    0x0c => {
                        // SUB reg - reg -> reg
                        do_math!(rrr overflowing_sub);
                    }
                    0x0d => {
                        // SUB zpr - reg -> reg
                        do_math!(zrr overflowing_sub);
                    }
                    0x0e => {
                        // SUB reg - zpr -> reg
                        do_math!(rzr overflowing_sub);
                    }
                    0x0f => {
                        // SUB reg - reg -> zpr
                        do_math!(rrz overflowing_sub);
                    }
                    0x10 => {
                        // SUB reg - zpr -> zpr
                        do_math!(rzz overflowing_sub);
                    }
                    0x11 => {
                        // SUB zpr - zpr -> zpr
                        do_math!(zzz overflowing_sub);
                    }
                    0x12 => {
                        // SUBb reg -b reg -> reg
                        do_math!(rrr borrowing_sub c);
                    }
                    0x13 => {
                        // SUBb zpr -b reg -> reg
                        do_math!(zrr borrowing_sub c);
                    }
                    0x14 => {
                        // SUBb reg -b zpr -> reg
                        do_math!(rzr borrowing_sub c);
                    }
                    0x15 => {
                        // SUBb reg - reg -> zpr
                        do_math!(rrz borrowing_sub c);
                    }
                    0x16 => {
                        // SUBb reg - zpr -> zpr
                        do_math!(rzz borrowing_sub c);
                    }
                    0x17 => {
                        // SUBb zpr - zpr -> zpr
                        do_math!(zzz borrowing_sub c);
                    }

                    _ => self.fault(INVALID_INSTRUCTION_VARIANT),
                }
            }
            0x05 => {
                // STACK instructions
                let v = self.mrn8_ip();
            }
            0x06 => {
                // IO instructions
                let v = self.mrn8_ip();
                match &v {
                    0x00 => {
                        // PRINT_CHAR Acc
                        self.clock();
                        print!(
                            "{}",
                            std::char::from_u32(self.acc).unwrap_or_else(|| {
                                self.fault(INVALID_CHAR);
                                ' '
                            })
                        );
                        io::stdout().flush().unwrap();
                    }
                    0x01 => {
                        // PRINT_CHAR reg
                        self.clock();
                        print!(
                            "{}",
                            std::char::from_u32(self.reg[self.mrn8_ip() as usize]).unwrap_or_else(
                                || {
                                    self.fault(INVALID_CHAR);
                                    ' '
                                }
                            )
                        );
                        io::stdout().flush().unwrap();
                    }
                    _ => self.fault(INVALID_INSTRUCTION_VARIANT),
                }
            }
            0x10 => self.reg[self.mrn8_ip() as usize] = self.reg[self.mrn8_ip() as usize], // MOV reg <= reg
            0x13 => {
                let r = self.mrn8_ip() as usize;
                self.reg[r >> 4] = self.reg[r & 0b1111]
            } // MOV 4br <= 4br
            0x14 => self.reg[self.mrn8_ip() as usize] = self.mrn8_ip() as u32, // CONST u8 -> reg
            0x15 => self.reg[self.mrn8_ip() as usize] = self.mrn16_ip() as u32, // CONST u16 -> reg
            0x16 => self.reg[self.mrn8_ip() as usize] = self.mrn32_ip() as u32, // CONST u32 -> reg

            0x20 => self.z = true,           // SET_ZF
            0x21 => self.z = false,          // CLR_ZF
            0x22 => self.c = true,           // SET_CF
            0x23 => self.c = false,          // CLR_CF
            0x24 => self.int_enable = true,  // INT_ENABLE
            0x45 => self.int_enable = false, // INT_DISABLE

            0x2c => self.reg[0] = self.reg[1], // MOV r1 -> r0
            0x2d => self.reg[1] = self.reg[0], // MOV r0 -> r1
            0x2e => self.reg[0] = self.reg[1], // MOV r2 -> r0
            0x2f => self.reg[1] = self.reg[2], // MOV r2 -> r1
            0x30 => {
                // ADD r0 + r1 -> r2
                let (n, o) = self.reg[0].overflowing_add(self.reg[1]);
                self.reg[2] = n;
                set_flags!(n, o);
            }
            0x31 => {
                // ADDc r0 + r1 -> r2
                let (n, o) = self.reg[0].carrying_add(self.reg[1], self.c);
                self.reg[2] = n;
                set_flags!(n, o);
            }
            0x32 => {
                // SUB r0 + r1 -> r2
                let (n, o) = self.reg[0].overflowing_sub(self.reg[1]);
                self.reg[2] = n;
                set_flags!(n, o);
            }
            0x33 => {
                // SUBb r0 + r1 -> r2
                let (n, o) = self.reg[0].borrowing_sub(self.reg[1], self.c);
                self.reg[2] = n;
                set_flags!(n, o);
            }
			0x34 => {
				// MUL r0 * r1 -> r2
				let (n, o) = self.reg[0].overflowing_mul(self.reg[1]);
				self.reg[2] = n;
				set_flags!(n, o);
			}
			0x35 => {
				// DIV r0 / r1 -> r2
				let (n, o) = self.reg[0].overflowing_div(self.reg[1]);
				self.reg[2] = n;
				set_flags!(n, o);
			}
			0x36 => {
				// AND r0 & r1 -> r2
				let n = self.reg[0] & self.reg[1];
				self.reg[2] = n;
				set_flags!(n, false);
			}
			0x37 => {
				// OR r0 & r1 -> r2
				let n = self.reg[0] | self.reg[1];
				self.reg[2] = n;
				set_flags!(n, false);
			}
			0x38 => {
				// XOR r0 & r1 -> r2
				let n = self.reg[0] ^ self.reg[1];
				self.reg[2] = n;
				set_flags!(n, false);
			}
			0x39 => {
				// NOT !r0 -> r2
				let n = !self.reg[0];
				self.reg[2] = n;
				set_flags!(n, false);
			}
			0x3a => {
				// DIV r0 / r1 -> r2
				let n = self.reg[0] % self.reg[1];
				self.reg[2] = n;
				set_flags!(n, false);
			}
			
            0xFE => {
                // HALT & EXIT
                self.fault(EXPLICIT_HALT_AND_EXIT);
            }
            0xFF => {
                // DEBUG INSTRUCTIONS
                let v = self.mrn8_ip();
                if self.print_exec {
                    println!("")
                }
                match &v {
                    0x00 => {
                        println!("\n{:08x?}", self.acc);
                    } // PRINT ACC
                    0x01 => {
                        println!("\n{:08x?}", self.reg[self.mrn8_ip() as usize])
                    } // PRINT reg
                    0x02 => {
                        for i in 0..16 {
                            println!("{:x?} = {:08x?}", i, self.reg[i])
                        }
                        println!("cf= {}", self.c);
                        println!("zf= {}", self.z);
                        println!("ac= {:08x?}", self.acc)
                    } // PRINT_ALL_REG
                    _ => self.fault(INVALID_INSTRUCTION_VARIANT),
                }
            }
            _ => self.fault(INVALID_INSTRUCTION),
        };
        flush_debug!();
    }
}
