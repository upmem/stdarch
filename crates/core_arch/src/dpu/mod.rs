//! DPU intrinsics

extern "C" {
    #[link_name = "llvm.dpu.tid.i32"]
    fn llvm_thread_id() -> i32;
}

#[inline]
pub unsafe fn thead_id() -> i32 {
    llvm_thread_id()
}
