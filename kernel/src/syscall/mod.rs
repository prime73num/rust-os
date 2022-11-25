const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;


mod fs;
mod provessor;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => fs::sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => provessor::sys_exit(args[0] as i32),
        SYSCALL_YIELD => provessor::sys_yield(),
        SYSCALL_GET_TIME => provessor::sys_get_time(),
        _ => panic!("Unsupported syscall_id!"),
    }
}
