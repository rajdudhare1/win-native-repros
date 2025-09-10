// By default, do NOT set the global allocator on Windows (jemalloc is fragile there).
#[cfg(any(not(target_os = "windows"), feature = "force-win"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    println!("jemalloc hello");
}