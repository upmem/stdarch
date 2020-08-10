//! DPU intrinsics

extern "C" {
    #[link_name = "llvm.dpu.tid.i32"]
    fn llvm_thread_id() -> i32;
    #[link_name = "llvm.dpu.ldma.unchecked"]
    fn llvm_mram_read(wram: *mut u8, mram: *const u8, len: u32);
    #[link_name = "llvm.dpu.sdma.unchecked"]
    fn llvm_mram_write(wram: *const u8, mram: *mut u8, len: u32);
}

#[inline]
pub unsafe fn thread_id() -> i32 {
    llvm_thread_id()
}

#[inline]
pub unsafe fn mram_read(wram: *mut u8, mram: *const u8, len: usize) {
    llvm_mram_read(wram, mram, len as u32)
}

#[inline]
pub unsafe fn mram_write(wram: *const u8, mram: *mut u8, len: usize) {
    llvm_mram_write(wram, mram, len as u32)
}
