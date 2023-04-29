use std::marker::PhantomData;

#[allow(dead_code)]
/*
 *  A guide to good APIs
 */

// Ergonomic traits implementations

/*
 *  Users will expect some default traits implementations.
 *  Including: Debug, Clone, Send, Sync.
 *
 *  On wrapper types: Deref, AsRef, Into<Inner> and From<Inner>
 */
#[derive(Debug, Clone)]
pub struct Pair<T> {
    x: T,
    y: T,
}
unsafe impl<T> Sync for Pair<T> {}
unsafe impl<T> Send for Pair<T> {}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub trait MyTrait {
    fn seila(&self) -> ();
}

impl<T> MyTrait for Pair<T> {
    fn seila(&self) -> () {
        ()
    }
}

/**
 * 1. Rust automatically will not implement traits
 *  for references of types.
 *
 *  Because the Trait may contain methods that require
 *  a &mut self or self. Which cannot be called on
 *  a reference.
 *
 *  To "fix" this, you'll want to create
 *  a blanket implementation of that type.
 *  using a &mut T, &T, or T.
 */
impl<T> MyTrait for &Pair<T> {
    fn seila(&self) -> () {
        ()
    }
}

pub fn foo(f: impl MyTrait) -> impl MyTrait {
    f
}

/*
 *  TYPE SAFETY: make your API hard to misuse or misuse-resistant.
 *  example: describing state using zero-sized types (unit structs).
 *  pattern: typestate
 */

pub struct Grounded;
pub struct Launched;

pub struct Rocket<State = Grounded> {
    state: PhantomData<State>,
    color: String,
}

impl Rocket {
    pub fn new(color: Option<String>) -> Self {
        Rocket {
            state: PhantomData::<Grounded>,
            color: color.unwrap_or("Red".to_string()),
        }
    }
}

// methods only available to Grounded
impl Rocket<Grounded> {
    #[must_use]
    pub fn launch(self) -> Rocket<Launched> {
        Rocket {
            state: PhantomData::<Launched>,
            color: self.color,
        }
    }
}

// methods only available to Launched
impl Rocket<Launched> {
    pub fn accelerate(&mut self) -> () {
        ()
    }
}

// methods available to all
impl<State> Rocket<State> {
    pub fn set_color(&mut self, c: String) {
        self.color = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rocket() {
        // only launch() available
        // Rocket<Grounded>
        let r = Rocket::new(Some("Blue".to_string()));
        // Rocket<Launch>
        let mut r = r.launch();
        r.accelerate();
        // accelerate() and others available
    }

    #[test]
    fn pair_works() {
        let pair = Pair::new(1, 1);
        let pair_ref = &pair;

        foo(pair.clone());
        // read 1.
        foo(pair_ref);

        pair_ref.seila();
        pair.seila();

        println!("{:?}", pair);
    }
}
