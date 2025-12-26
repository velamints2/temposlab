use alloc::sync::Arc;
use log::debug;
use ostd::{Pod, mm::Vaddr};

use crate::error::{Errno, Error, Result};
use crate::process::Process;
use crate::syscall::SyscallReturn;

const RLIM_INFINITY: u64 = u64::MAX;

#[derive(Debug, Clone, Copy, Pod)]
#[repr(C)]
pub struct RLimit64 {
    cur: u64,
    max: u64,
}

const RLIMIT_STACK: i32 = 3;
const RLIMIT_AS: i32 = 9;

pub fn sys_prlimit64(
    pid: i32,
    resource: i32,
    _new_limit: Vaddr,
    old_limit: Vaddr,
    current_process: &Arc<Process>,
) -> Result<SyscallReturn> {
    if pid != 0 {
        return Err(Error::new(Errno::EINVAL));
    }

    debug!(
        "[SYS_PRLIMIT64] pid: {}, resource: {}, new_limit: {:#x}, old_limit: {:#x}",
        pid, resource, _new_limit, old_limit
    );

    let mut rlim = RLimit64 {
        cur: RLIM_INFINITY,
        max: RLIM_INFINITY,
    };

    if resource == RLIMIT_STACK {
        rlim.cur = 8 * 1024 * 1024;
        rlim.max = 8 * 1024 * 1024;
    }

    if old_limit != 0 {
        current_process
            .memory_space()
            .vm_space()
            .writer(old_limit, core::mem::size_of::<RLimit64>())
            .unwrap()
            .write_val(&rlim)
            .unwrap();
    }

    Ok(SyscallReturn(0))
}
