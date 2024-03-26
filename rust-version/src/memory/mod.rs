pub mod bump_allocator;
pub mod initramfs;


#[global_allocator]
pub static ALLOCATOR: bump_allocator::BumpAllocator = bump_allocator::BumpAllocator::new();
// pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();

