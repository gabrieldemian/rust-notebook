#[allow(dead_code)]
/*
*
*   Lifetimes
*   lifetimes are named regions of code that a reference
*   must be valid for. Those regions correspond to paths
*   of execution in the code. Most of the times the lifetimes
*   will coincide with the scope, but they are not the same.
*
*   rust will always try to elid the lifetimes(infer) whenever possible.
*   if your references are too complex and dont follow the `elision rules`
*   the compiler will ask you to manually manage the lifetimes.
*
*   A reference (sometimes called a borrow) is alive from the place it is
*   created to its last use. The borrowed value needs to outlive only borrows that are alive.
*
*  Lifetime Elission Rules
*  When possible, Rust will always elid lifetimes in function signatures.
*
*  Rules for ellisions:
*  - Each elided lifetime in input position becomes a distinct lifetime parameter. read: str_tok,
*  str_tok
*  - If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes. read: ouch_2
*  - If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes. read: Ouch struct
*  - Otherwise, it is an error to elide an output lifetime. read: ouch_3
*/

// 1.
// pub fn str_tok<'a>(x: &'a mut &'a str, delimiter: char) -> &'a str {
pub fn str_tok<'a, 'b>(x: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = x.find(delimiter) {
        let prefix = &x[..i];
        let suffix = &x[(i + delimiter.len_utf8())..];
        *x = suffix;
        return prefix;
    };
    let prefix = *x;
    *x = "";
    return prefix;
}
/*
 *  At (1), if there is only one lifetime, 'a for `x` and `delimiter`
 *  the compiler infers that 'a is 'static, because `x` is &'static str.
 *  therefore making the code to not compile. because x is a mutable reference
 *  that would live throughout the entire program, and so, it could not be
 *  immutably referenced again.
 *
 *  if there are 2 lifetimes, 'a and 'b.
 *  'a is set to the to-be-determined lifetime of the
 *  borrowed `x`. Which is now used and destroyed inside the fn.
 *  And not 'static like it could be if there were only one lifetime.
 *  'b is set to 'static since that is the lifetime of `x`.
 */

// 2.
pub fn ouch_2(_x: &String) -> &String {
    _x
}
// will de-sugar to
// ```rs
// pub fn ouch_2<'a>(_x: &'a String) -> &'a String {
//     _x
// }
// ```

// here, the lifetime of the input is assigned to all
// ellided (hidden) output lifetimes
pub fn ouch_2_1(_x: &str) -> (&str, &str) {
    ("x", "y")
}

#[derive(Debug)]
pub struct Ouch;
impl Ouch {
    // whoever calls this method, will have a
    // reference that is valid until the struct
    // is valid, because of the lifetime elision rules.
    pub fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    // desugars to:
    // ```rs
    // pub fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
    //     &*self
    // }
    // ```
    pub fn share(&self) {}
}

// example of lifetime bound
// this wont compile without lifetimes,
// because rust cant know the relationship
// between the lifetimes. Or who lives as much
// or longer than who.
pub fn ouch_4<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    // here i'm declaring that
    // 'b is a subtype of 'a
    // 'b lives at least as long as 'a
    // where 'longer outlives 'shorter
    'b: 'a,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifetime_colision() {
        let mut ouch = Ouch; // 'a mutable
                             // loan lives as much as ouch lives
        let loan = ouch.mutate_and_share(); // 'a
                                            // lifetime colision (interpolation) here, ouch is
                                            // borrowed as mutable and immutable
                                            // at the same time
        ouch.share(); // 'b immutable AND 'a mutable = collision
                      // print!("{:?}", loan); // 'a mutable
                      // uncomment to read the error /\
    }

    #[test]
    fn hello_world() {
        // &'static
        let mut x = "hello world";
        let hello = str_tok(&mut x, ' '); // &'a mut x

        {
            println!("{x}"); // 'a
            println!("{hello}");
        }

        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}

/*
 * Subtype:
 * 'static is a subtype of 'a
 * because it lives at least as long as 'a
 * or, it is as useful as 'a
 *
 * another example:
 * if 'b: 'a
 * if 'b outlives 'a, then 'b is a subtype of 'a
 */

/*
*  All types have a variance, which defines what other similar types
*  can be used in that's type place.
*
*  1. covariant.
*      if you can just use a subtype in place of the type.
*  2. invariant.
*      if the type requires the same type.
*  3. contravariant.
*      this comes up for functions arguments.
*      the more useful the variable, the less useful the function
*/
