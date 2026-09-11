use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

struct ListNode {
    size: usize,
    next: *mut ListNode,
}

impl ListNode {
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
            head: ListNode {
                size: 0,
                next: ptr::null_mut(),
            },
        }
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        unsafe {
            let head_ptr = &mut self.head as *mut ListNode;

            let mut prev = head_ptr;
            while !(*prev).next.is_null() && (*(*prev).next).start_addr() < addr {
                prev = (*prev).next;
            }
            let next = (*prev).next;

            let node_ptr = addr as *mut ListNode;
            node_ptr.write(ListNode { size, next });
            (*prev).next = node_ptr;

            if !next.is_null() && (*node_ptr).end_addr() == (*next).start_addr() {
                (*node_ptr).size += (*next).size;
                (*node_ptr).next = (*next).next;
            }

            if prev != head_ptr && (*prev).end_addr() == (*node_ptr).start_addr() {
                (*prev).size += (*node_ptr).size;
                (*prev).next = (*node_ptr).next;
            }
        }
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(*mut ListNode, usize)> {
        unsafe {
            let mut prev = &mut self.head as *mut ListNode;

            loop {
                let current = (*prev).next;
                if current.is_null() {
                    return None;
                }

                if let Ok(alloc_start) = Self::region_fits(&*current, size, align) {
                    (*prev).next = (*current).next;
                    return Some((current, alloc_start));
                }

                prev = current;
            }
        }
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

        if let Some((region_ptr, alloc_start)) = list.find_region(size, align) {
            let region_end = unsafe { (*region_ptr).end_addr() };
            let alloc_end = alloc_start + size;
            let excess = region_end - alloc_end;
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
