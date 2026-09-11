use crate::kprintln;
use alloc::string::String;

pub fn exec() {
    let vendor = get_vendor();
    kprintln!("{}", vendor);
}

pub fn get_vendor() -> String {
    let mut ebx_out: u32;
    let mut edx: u32;
    let mut ecx: u32;

    unsafe {
        core::arch::asm!(
            "mov r8, rbx",
            "cpuid",
            "mov r9, rbx",
            "mov rbx, r8",
            inout("eax") 0 => _,
            out("r8") _,
            out("r9") ebx_out,
            out("ecx") ecx,
            out("edx") edx,
            options(nomem, nostack));
    }

    let mut bytes = [0u8; 12];

    bytes[0..4].copy_from_slice(&ebx_out.to_le_bytes());
    bytes[4..8].copy_from_slice(&edx.to_le_bytes());
    bytes[8..12].copy_from_slice(&ecx.to_le_bytes());

    String::from_utf8_lossy(&bytes).into_owned()
}
