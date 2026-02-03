use libc::user_regs_struct;

pub struct Registers {
    pub inner: user_regs_struct,
}

impl Registers {
    pub fn new(inner: user_regs_struct) -> Self {
        Self { inner }
    }

    pub fn syscall_number(&self) -> u64 {
        self.inner.orig_rax
    }

    pub fn return_value(&self) -> u64 {
        self.inner.rax
    }

    pub fn arg1(&self) -> u64 { self.inner.rdi }
    pub fn arg2(&self) -> u64 { self.inner.rsi }
    pub fn arg3(&self) -> u64 { self.inner.rdx }
    pub fn arg4(&self) -> u64 { self.inner.r10 }
    pub fn arg5(&self) -> u64 { self.inner.r8 }
    pub fn arg6(&self) -> u64 { self.inner.r9 }
}
