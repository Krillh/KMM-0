#![feature(bigint_helper_methods)]
#![feature(mixed_integer_ops)]

mod emulator;
mod assembler;

fn block_print(width: usize, nums: Vec<u8>) {
	for i in 0..nums.len() {
		if i % width == 0 {
			println!();
		}
		print!("{:02x?} ", nums[i]);
	}
	println!();
}

macro_rules! skip256bytes {
	() => {
		vec![
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,]
	};
}

const R0: u8 = 0x0;
const R1: u8 = 0x1;
const R2: u8 = 0x2;
const R3: u8 = 0x3;
const R4: u8 = 0x4;
const R5: u8 = 0x5;
const R6: u8 = 0x6;
const R7: u8 = 0x7;
const IP: u8 = 0x8; // instruction pointer
const SP: u8 = 0x9; // stack pointer
const RP: u8 = 0xa; // interrupt return pointer
const TR: u8 = 0xb; // temp register
const IX: u8 = 0xc; // index register

const NOP: u8 = 0x00;
const JMP: u8 = 0x02;
	const JMP_24: u8 = 0x00;
	const JMP_32: u8 = 0x01;
	const JMP_P_OFFSET_8: u8 = 0x02;
	const JMP_S_OFFSET_8: u8 = 0x03;
	const JMP_P_OFFSET_16: u8 = 0x04;
	const JMP_S_OFFSET_16: u8 = 0x05;
	const JMP_P_OFFSET_32: u8 = 0x06;
	const JMP_S_OFFSET_32: u8 = 0x07;
const JMPC: u8 = 0x03;
	const  JC_24: u8 = 0x00;
	const  JC_32: u8 = 0x01;
	const  JC_P_OFFSET_8: u8 = 0x02;
	const  JC_S_OFFSET_8: u8 = 0x03;
	const  JC_P_OFFSET_16: u8 = 0x04;
	const  JC_S_OFFSET_16: u8 = 0x05;
	const  JC_P_OFFSET_32: u8 = 0x06;
	const  JC_S_OFFSET_32: u8 = 0x07;
	const  JNC_24: u8 = 0x08;
	const  JNC_32: u8 = 0x09;
	const  JNC_P_OFFSET_8: u8 = 0x0a;
	const  JNC_S_OFFSET_8: u8 = 0x0b;
	const  JNC_P_OFFSET_16: u8 = 0x0c;
	const  JNC_S_OFFSET_16: u8 = 0x0d;
	const  JNC_P_OFFSET_32: u8 = 0x0e;
	const  JNC_S_OFFSET_32: u8 = 0x0f;
	const  JZ_24: u8 = 0x10;
	const  JZ_32: u8 = 0x11;
	const  JZ_P_OFFSET_8: u8 = 0x12;
	const  JZ_S_OFFSET_8: u8 = 0x13;
	const  JZ_P_OFFSET_16: u8 = 0x14;
	const  JZ_S_OFFSET_16: u8 = 0x15;
	const  JZ_P_OFFSET_32: u8 = 0x16;
	const  JZ_S_OFFSET_32: u8 = 0x17;
	const  JNZ_24: u8 = 0x18;
	const  JNZ_32: u8 = 0x19;
	const  JNZ_P_OFFSET_8: u8 = 0x1a;
	const  JNZ_S_OFFSET_8: u8 = 0x1b;
	const  JNZ_P_OFFSET_16: u8 = 0x1c;
	const  JNZ_S_OFFSET_16: u8 = 0x1d;
	const  JNZ_P_OFFSET_32: u8 = 0x1e;
	const  JNZ_S_OFFSET_32: u8 = 0x1f;
const MATH: u8 = 0x04;
	const ADD_RRR: u8 = 0x00;
	const ADD_ZRR: u8 = 0x01;
	const INC_1: u8 = 0x02;
	const ADD_RRZ: u8 = 0x03;
	const ADD_RZZ: u8 = 0x04;
	const ADD_ZZZ: u8 = 0x05;
	const ADDC_RRR: u8 = 0x06;
	const ADDC_ZRR: u8 = 0x07;
	const INC_4: u8 = 0x08;
	const ADDC_RRZ: u8 = 0x09;
	const ADDC_RZZ: u8 = 0x0a;
	const ADDC_ZZZ: u8 = 0x0b;
const CONST8: u8 = 0x14;
const EXPLICIT_HALT_AND_EXIT: u8 = 0xFE;
const DEBUG: u8 = 0xFF;
	const PRINT_ACC: u8 = 0x00;
	const PRINT_REG: u8 = 0x01;
	const PRINT_ALL_REG: u8 = 0x02;

use assembler::Instruction as I;
use assembler::{Assembler, Ver0};

fn main() {
	let mut skip: Vec<u8> = skip256bytes!();
	// let mut code: Vec<u8> = vec![
	// 	CONST8, 0xFF, R0,             // CONST 0xFF => r0
	// 	CONST8, 0x07, R1,             // CONST r1 = 0x01
	// 	MATH, ADD_RRR, R0, R1, R0,          // ADD r0 = r0 + r1
	// 	DEBUG, PRINT_REG, R0,		        // DEBUG_PRINT_REG r0
	// 	DEBUG, PRINT_REG, R1,		        // DEBUG_PRINT_REG r1
	// 	JMPC, JNC_S_OFFSET_8, 0x0d,		    // JNC-offset 9
	// 	DEBUG, PRINT_REG, R0,		        // DEBUG_PRINT_ALL_REG
	// 	EXPLICIT_HALT_AND_EXIT,			    // EXPLICIT HALT AND EXIT
	// ];
	let instruction_tokens: Vec<I> = vec![
		I::ConstRegU8(R0, 0xFF),
		I::ConstRegU8(R1, 0x07),
		I::AddRRR(R0, R1, R0),
		I::DebugPrintReg(R0),
		I::DebugPrintReg(R1),
		I::JNCOffsetS8(0x0d),
		I::DebugPrintReg(R0),
		I::ExplicitHaltAndExit,
	];
	let code = Ver0::assemble_to_bytes(instruction_tokens);
	block_print(16, code);
}