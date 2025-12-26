use alloc::sync::Arc;
use ostd::mm::PageFlags;
use crate::error::Result;
use crate::process::Process;
use crate::syscall::SyscallReturn;

pub const PROT_NONE: usize = 0;
pub const PROT_READ: usize = 1;
pub const PROT_WRITE: usize = 2;
pub const PROT_EXEC: usize = 4;

pub fn sys_mprotect(
    addr: usize,
    len: usize,
    prot: usize,
    current_process: &Arc<Process>,
) -> Result<SyscallReturn> {
    let memory_space = current_process.memory_space();
    let new_flags = translate_prot_flags(prot);

    memory_space.protect(addr, len, new_flags)?;

    Ok(SyscallReturn(0))
}

fn translate_prot_flags(prot: usize) -> PageFlags {
    let mut flags = PageFlags::U | PageFlags::V;
    if prot & PROT_READ != 0 {
        flags |= PageFlags::R;
    }
    if prot & PROT_WRITE != 0 {
        flags |= PageFlags::W;
    }
    if prot & PROT_EXEC != 0 {
        flags |= PageFlags::X;
    }
    flags
}

