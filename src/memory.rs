use std::mem;
use std::ops::{Index, IndexMut};
use region::{View, Protection};
use libc;
use aligned_alloc;

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

const PAGE_SIZE: usize = 4096;

pub struct Memory {
    contents: *mut u8,
    size: usize,
    pc: usize,
    ptr: *mut (),
}

impl Memory {
    pub fn new(num_pages: usize) -> Memory {
        let contents: *mut u8;
        let size = num_pages * PAGE_SIZE;
        let mut ptr: *mut libc::c_void;
        unsafe {
            ptr = aligned_alloc::aligned_alloc(PAGE_SIZE, size) as *mut libc::c_void;
            let mut view = View::new(ptr as *const u8, size).unwrap();
            view.set_prot(Protection::ReadWriteExecute.into()).unwrap();
            memset(ptr, 0xc3, size); // fill up with RET for security reasons

            contents = mem::transmute(ptr);
        }
        let _ptr = ptr as *mut ();
        Memory {
            contents,
            size,
            pc: 0,
            ptr: _ptr,
        }
    }

    pub fn fill(&mut self, asm: u8) {
        for i in 0..self.size {
            self[i] = asm;
        }
    }

    pub fn dump(&self) {
        for i in 0..100 {
            print!("{:02X} ", self[i]);
            if (i + 1) % 10 == 0 {
                println!("");
            }
        }
        println!("\n");
    }

    pub fn reset(&mut self) {
        self.fill(0xc3);
        self.pc = 0;
    }

    pub fn emit(&mut self, asm: u8) {
        let pc = self.pc;
        self[pc] = asm;
        self.pc += 1;
    }

    pub fn emit_bytes(&mut self, asm: Vec<u8>) {
        for b in asm {
            self.emit(b);
        }
    }

    pub fn emit32(&mut self, asm: u32) {
        self.emit((asm & 0xFF) as u8);
        self.emit(((asm >> 8) & 0xFF) as u8);
        self.emit(((asm >> 16) & 0xFF) as u8);
        self.emit(((asm >> 24) & 0xFF) as u8);
    }

    pub fn emit64(&mut self, asm: u64) {
        self.emit32((asm & 0xFFFFFFFF) as u32);
        self.emit32(((asm >> 32) & 0xFFFFFFFF) as u32);
    }

    pub fn execute(&self) -> usize {
        let func: (fn() -> usize);
        unsafe {
            func = mem::transmute(self.contents);
        }

        func()
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.contents.offset(_index as isize) }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            aligned_alloc::aligned_free(self.ptr);
        }
    }
}