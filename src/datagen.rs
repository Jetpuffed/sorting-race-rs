use core::arch::x86_64::
{
    __cpuid,
    _rdrand32_step,
};

use std::
{
    sync::mpsc,
    thread,
};

/// Uses the x86-64 instruction `RDRAND` to randomly
/// generate a 32-bit unsigned integer from hardware.
fn hw_rand_u32() -> u32
{
    let mut n = 0;
    unsafe { _rdrand32_step(&mut n) };

    return n
}

/// Uses the x86-64 instruction `CPUID` to obtain
/// the amount of logical cores in the current machine.
pub fn get_logical_cores() -> usize
{
    // Selects leaf EAX=1 and gets "additional information" from register EBX
    let cpuid = unsafe { __cpuid(1).ebx };

    // Isolate bits 16 ..= 23 with bitwise AND then right shift
    // by 16 to get the maximum number of addressable logical cores.
    return ((cpuid & 0xFF0000) >> 16) as usize
}

pub fn generate_data() -> Vec<u32>
{
    // Number of threads to use for generating data
    let threads: usize = get_logical_cores() - 1;

    // total_size * 32 = 1 MiB
    let total_size: usize = 32768;

    // If running on a single-core machine, fall back to single threaded mode
    if threads == 0
    {
        // Pre-allocate memory for a vector with capacity
        // `total_size` to store the randomly generated numbers
        let mut rd_vec = Vec::with_capacity(total_size);

        // Generate a random number and push it to the vector
        for _ in 0 .. total_size
        {
            rd_vec.push(hw_rand_u32());
        }

        return rd_vec
    }

    // If running on a multi-core (and/or multi-threaded) machine,
    // generate data using all cores/threads available
    else
    {
        // Pre-allocate memory for a vector with capacity
        // `total_size` to store the randomly generated numbers
        let mut rd_vec = Vec::with_capacity(total_size);

        // Pre-allocate memory for a vector with capacity
        // `threads` to store the jobs for each worker (thread)
        let mut workers = Vec::with_capacity(threads);

        // Divide `total_size` by the amount of threads to get the job size for each worker.
        let job_size = total_size / threads;

        // Create a new channel for the transmitters (worker threads)
        // to send data to the receiver (main thread) for processing
        let (tx, rx) = mpsc::channel();

        // Create the jobs and then push the job to the worker pool
        for _ in 0 .. threads
        {
            // Create a clone of the transmitter for the worker
            let tx = tx.clone();

            // Create a new job for a worker thread
            let worker = thread::spawn(
                move ||
                {
                    // Pre-allocate memory for a vector with capacity
                    // `job_size` to temporarily store generated data
                    let mut tmp = Vec::with_capacity(job_size);

                    // Generate the random numbers
                    for _ in 0 .. job_size
                    {
                        tmp.push(hw_rand_u32());
                    }

                    // Send the data stored in `tmp` to the receiver for processing
                    tx.send(tmp).unwrap();
                }
            );

            // Push the job to the worker pool
            workers.push(worker);
        }

        // Allow the workers to begin their jobs
        for worker in workers
        {
            worker.join().unwrap();
        }

        // While workers are busy with their jobs, the main thread will
        // be busy processing the received data and preparing the return value
        while let Ok(recv) = rx.try_recv()
        {
            // Iterate through the received packaged data
            // and push it to the return value `rd_vec`
            for rd_u32 in recv
            {
                rd_vec.push(rd_u32);
            }
        }

        return rd_vec
    }
}
