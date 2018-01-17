extern crate libc;
extern crate aligned_alloc;
extern crate region;

mod memory;

use memory::Memory;
use std::mem;

fn modify_var() {
    let value = 10;
    println!("Before: {:?}", value);
    let address = &value as *const _;
    println!("Address: {:?}", address);
    let mut memory = Memory::new(1);
    memory.emit_bytes(vec![0x49, 0xBC]);
    memory.emit64(address as u64); // movabs r12, <u64 address>
    memory.emit_bytes(vec![0x49, 0x83, 0x04, 0x24, 0x0A]); // add qword ptr [r12], 0xa 
    memory.dump();
    let _ = memory.execute();
    println!("After: {:?}", value);
    drop(memory);
}

fn modify_array() {
    let mut memory = Memory::new(1);
    let arr = [1, 2, 3];
    let address = &arr as *const _;
    println!("Before: {:?}", arr);
    println!("Address: {:?}", address);
    memory.emit_bytes(vec![0x49, 0xBC]);
    memory.emit64(address as u64 + 0); // movabs r12, <u64 address>; 0 is the index
    memory.emit_bytes(vec![0x49, 0x83, 0x04, 0x24, 0x0A]); // add qword ptr [r12], 0xa 
    memory.dump();
    let _ = memory.execute();
    println!("After: {:?}", arr);
    drop(memory);
}

fn modify_array_generic() {
    let mut memory = Memory::new(1);
    let arr = [1 as u64, 2 as u64, 3 as u64];
    let address = &arr as *const _;
    println!("Before: {:?}", arr);
    println!("Address: {:?}", address);
    memory.emit_bytes(vec![0x49, 0xBC]);
    memory.emit64((address as usize + (mem::size_of_val(&arr[0])) * 2) as u64); // movabs r12, <u64 address>; 2 is the index
    memory.emit_bytes(vec![0x49, 0x83, 0x04, 0x24, 0x0A]); // add qword ptr [r12], 0xa 
    memory.dump();
    let _ = memory.execute();
    println!("After: {:?}", arr);
    drop(memory);
}

fn main() {
    modify_var();
    modify_array();
    modify_array_generic();
    loop {}
}