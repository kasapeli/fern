use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

struct Locked<T> {
    locked: AtomicBool,
    inner: core::cell::UnsafeCell<T>,
}

unsafe impl<T> Sync for Locked<T> {}

impl<T> Locked<T> {
    const fn new(inner: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            inner: core::cell::UnsafeCell::new(inner),
        }
    }

    fn lock(&self) -> LockedGuard<T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        LockedGuard { lock: self }
    }
}

struct LockedGuard<'a, T> {
    lock: &'a Locked<T>,
}

impl<'a, T> core::ops::Deref for LockedGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.inner.get() }
    }
}

impl<'a, T> core::ops::DerefMut for LockedGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.inner.get() }
    }
}

impl<'a, T> Drop for LockedGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

struct FreeList {
    head: ListNode,
}

impl FreeList {
    const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        unsafe {
            node_ptr.write(node);
            self.head.next = Some(&mut *node_ptr);
        }
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::region_fits(region, size, align) {
                let next = region.next.take();
                let region = current.next.take().unwrap();
                current.next = next;
                return Some((region, alloc_start));
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        None
    }

    fn region_fits(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }
        let excess = region.end_addr() - alloc_end;
        if excess > 0 && excess < mem::size_of::<ListNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

fn size_align(layout: Layout) -> (usize, usize) {
    let layout = layout
        .align_to(mem::align_of::<ListNode>())
        .expect("adjusting alignment failed")
        .pad_to_align();
    let size = layout.size().max(mem::size_of::<ListNode>());
    (size, layout.align())
}

pub struct Allocator {
    list: Locked<FreeList>,
}

unsafe impl Sync for Allocator {}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            list: Locked::new(FreeList::new()),
        }
    }

    pub unsafe fn init(&self, start: usize, size: usize) {
        let start = align_up(start, mem::align_of::<ListNode>());
        unsafe {
            self.list.lock().add_free_region(start, size);
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = size_align(layout);
        let mut list = self.list.lock();

        if let Some((region, alloc_start)) = list.find_region(size, align) {
            let alloc_end = alloc_start + size;
            let excess = region.end_addr() - alloc_end;
            if excess > 0 {
                unsafe {
                    list.add_free_region(alloc_end, excess);
                }
            }
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = size_align(layout);
        unsafe {
            self.list.lock().add_free_region(ptr as usize, size);
        }
    }
}
