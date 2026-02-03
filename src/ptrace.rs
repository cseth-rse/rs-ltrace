use libc::*;
use crate::error::TraceError;
use std::mem;
use std::ptr;

pub fn attach(pid: pid_t) -> Result<(), TraceError> {
    let res = unsafe { ptrace(PTRACE_ATTACH, pid, ptr::null_mut::<c_void>(), 0) };
    if res == -1 {
        return Err(TraceError::AttachFailed);
    }
    Ok(())
}

pub fn cont(pid: pid_t) -> Result<(), TraceError> {
    let res = unsafe { ptrace(PTRACE_SYSCALL, pid, 0, 0) };
    if res == -1 {
        return Err(TraceError::PtrError("Failed to continue".into()));
    }
    Ok(())
}

pub fn get_registers(pid: pid_t) -> Result<user_regs_struct, TraceError> {
    let mut regs: user_regs_struct = unsafe { mem::zeroed() };

    let res = unsafe {
        ptrace(
            PTRACE_GETREGS,
            pid,
            ptr::null_mut::<c_void>(),
            &mut regs,
        )
    };

    if res == -1 {
        return Err(TraceError::PtrError("Failed to get registers".into()));
    }

    Ok(regs)
}

pub fn wait(pid: pid_t) -> Result<i32, TraceError> {
    let mut status: i32 = 0;
    let res = unsafe { waitpid(pid, &mut status, 0) };
    if res == -1 {
        return Err(TraceError::WaitFailed);
    }
    Ok(status)
}

pub fn detach(pid: pid_t) {
    unsafe {
        ptrace(PTRACE_DETACH, pid, 0, 0);
    }
}
