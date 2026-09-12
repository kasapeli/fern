use crate::HEAP_MEM;
use crate::kprintln;

pub fn exec() {
    kprintln!("{}", HEAP_MEM.len());
}
