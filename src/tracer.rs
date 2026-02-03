use libc::*;
use crate::ptrace;
use crate::registers::Registers;
use crate::error::TraceError;
use std::ffi::CString;
use std::ptr;

pub struct Tracer {
    program: Vec<String>,
    entering: bool,
}

impl Tracer {
    pub fn new(program: Vec<String>) -> Self {
        Self {
            program,
            entering: true,
        }
    }

    pub fn run(&mut self) -> Result<(), TraceError> {
        unsafe {
            let pid = fork();

            if pid == -1 {
                return Err(TraceError::PtrError("Fork failed".into()));
            }

            if pid == 0 {
                // --- CHILD ---
                ptrace(PTRACE_TRACEME, 0, ptr::null_mut::<c_void>(), 0);

                let cstrings: Vec<CString> = self.program
                    .iter()
                    .map(|arg| CString::new(arg.as_str()).unwrap())
                    .collect();

                let mut c_ptrs: Vec<*const i8> =
                    cstrings.iter().map(|s| s.as_ptr()).collect();

                c_ptrs.push(ptr::null());

                execvp(c_ptrs[0], c_ptrs.as_ptr());

                std::process::exit(1);
            } else {
                // --- PARENT ---
                let mut status: i32 = 0;
                waitpid(pid, &mut status, 0);

                // First stop after exec
                ptrace(PTRACE_SYSCALL, pid, 0, 0);

                loop {
                    waitpid(pid, &mut status, 0);

                    if WIFEXITED(status) {
                        println!("Process exited.");
                        break;
                    }

                    if WIFSTOPPED(status) {
                        let regs = ptrace::get_registers(pid)?;
                        let regs = Registers::new(regs);

                        if self.entering {
                            println!(
                                "syscall({}) args: {}, {}, {}",
                                regs.syscall_number(),
                                regs.arg1(),
                                regs.arg2(),
                                regs.arg3()
                            );
                        } else {
                            println!(" → return = {}", regs.return_value());
                        }

                        self.entering = !self.entering;

                        ptrace(PTRACE_SYSCALL, pid, 0, 0);
                    }
                }
            }
        }

        Ok(())
    }
}
