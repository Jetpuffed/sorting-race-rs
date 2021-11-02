use core::arch::x86_64::*;

pub fn get_rand_u64() -> u64
{
    let mut val: u64 = 0;
    unsafe { _rdseed64_step(&mut val) };

    return val
}

pub fn get_rand_u64_vec(n: usize) -> Vec<u64>
{
    let mut dst: Vec<u64> = Vec::with_capacity(n);

    for _ in 0 .. n
    {
        dst.push(get_rand_u64());
    }

    return dst
}
