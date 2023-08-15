use std::alloc::Allocator;

#[must_use]
pub fn alloc_buffer<T: Default>(size: usize) -> Box<[T]> {
    (0..size).map(|_| T::default()).collect()
}

#[must_use]
pub fn alloc_buffer_in<T, A>(
    size: usize,
    alloc: A,
) -> Box<[T], A>
where
    A: Allocator,
    T: Default,
{
    let mut b = Box::new_uninit_slice_in(size, alloc);
    for i in 0..size {
        b[i].write(T::default());
    }
    // SAFETY: All allocated memory has just been initialized
    unsafe { b.assume_init() }
}
