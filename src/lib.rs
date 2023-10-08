use wee_alloc::WeeAlloc;

pub mod game;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
