#[derive(Clone, Copy)]
pub(crate) struct FnPtr<T>(pub fn(&mut T, Vec<String>));

impl<T> std::fmt::Debug for FnPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&(self.0 as usize as *const ()), f)
    }
}
