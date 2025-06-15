/// Return the remaining compute units the program may consume
//In Solana programs, target_os = "solana" is a custom target set up when compiling to BPF (the on-chain VM).
#[inline]
pub fn sol_remaining_compute_units() -> u64 {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_remaining_compute_units()
    }

    #[cfg(not(target_os = "solana"))]
    {
        crate::program_stubs::sol_remaining_compute_units()
    }
}
