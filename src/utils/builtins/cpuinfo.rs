use crate::println;

#[derive(Debug)]
pub struct CpuVendor {
    pub name: [u8; 12],
}

pub fn exec() {
    let vendor = get_vendor();
    if let Ok(str) = core::str::from_utf8(&vendor.name) {
        println!("{}", str);
    } else {
        println!("unknown");
    }
}

pub fn get_vendor() -> CpuVendor {
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

    let mut name = [0u8; 12];

    name[0..4].copy_from_slice(&ebx_out.to_le_bytes());
    name[4..8].copy_from_slice(&edx.to_le_bytes());
    name[8..12].copy_from_slice(&ecx.to_le_bytes());

    CpuVendor { name }
}
