/// base
pub mod rwlock_1;
/// Avoid busy-looping writers:
/// If lots of readers, the writer won't wait because `s` will change before syscall wait
pub mod rwlock_2;
/// Avoiding Writer Starvation:
/// Idea: reader counter, odd => writer waiting, even => only reader
///                       writer will increment by 1, and reader by 2
pub mod rwlock_3;
