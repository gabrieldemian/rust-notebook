/*
 * since this fn is "static dispatched". The compiler will
 * create a new copy of this fn for each type that uses it
 * in this case, 2 copies: one for &str and other for String
 * this process is called monomorphization
 */
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

/*
 *  dyn does not implement the trait `Sized`
 *  so it must be behind a type (pointer) that implements Sized
 *  in order to compile
 *  e.g: a &, Box (implements Sized), or Arc.
 */
pub fn strlen2(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

// this would make the entire trait to not be
// used by a trait object
// pub trait Hei where Self: Sized {
pub trait Hei {
    type Item;

    fn hei(&self);
    // cannot be called from a trait object,
    // because they don't implement Sized
    fn weird(&self) -> Self::Item
    where
        // "Self" refers to the type that
        // implements this trait "Hei"
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_test() {
        //      &'static str
        strlen("hello world");
        /* This String is accepted because String implements
         * AsRef>str>. It can be converted to a &str
         */
        //      String: AsRef<str>
        strlen("hallo welt".to_string());
    }

    #[test]
    fn hrlt() {
        quox(|x| x);
    }
}
