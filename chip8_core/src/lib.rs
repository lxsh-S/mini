use std::ops::StartInclusive;

const RAM_SIZE: usize = 4096; // 4kb
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE], //array of u8 defined by RAM_SIOZE
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Array of bools defined by scren width and
    // height
    v_reg: [u8; NUM_REGS], // v rgister
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

const START_ADDR: u16 = 0x200; //512 bytes

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }
}
