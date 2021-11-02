use core::arch::x86_64::*;

pub fn get_rand_u64() -> u64
{
    let mut val: u64 = 0;
    unsafe { _rdseed64_step(&mut val) };

    return val
}
