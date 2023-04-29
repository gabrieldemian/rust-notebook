#[allow(dead_code)]
/*
 *  Lifetimes and variance
 */
// 1.
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
 *  At (1), if there is only one lifetime, only 'a for both `x` and `delimiter`
 *  the compiler infers that this lifetime 'a is 'static, because `x` is &'static str.
 *  therefore making the code to not compile.
 *
 *  if there are 2 lifetimes, 'a and 'b.
 *  'a is set to the to-be-determined lifetime of the
 *  borrowed `x`. Which is now used and destroyed inside the fn.
 *  And not 'static like it could be if there were only one lifetime.
 *  'b is set to 'static since that is the lifetime of `x`.
 */

// 2.
// by default the compiler would try to assign the input lifetime
// to the output lifetimes. But since we have 2 inputs here
// and each inputs has a distinct lifetime... we need to specify which input
// lifetime we want to make the return valid for.
// in this case, for 'a
pub fn ouch<'a, 'b>(_x: &'a str, _y: &'b str) -> &'a str {
    ""
}

// the lifetime of the input is assigned to all
// ellided (hidden) output lifetimes
pub fn ouch2(_x: &str) -> (&str, &str) {
    ("x", "y")
}

// example of lifetime bound
pub fn ouch3<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    // where 'longer outlives 'shorter
    'b: 'a,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
*  At (2). When possible, Rust will always elid (hide) lifetimes
*  in function signatures.
*
*  Rules for ellisions:
*  - Each elided lifetime in input position becomes a distinct lifetime parameter. read: ouch
*  - If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes. read: ouch2
*  - If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.
*  - Otherwise, it is an error to elide an output lifetime. read: ouch3
*/

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

*  1. covariant.
*      if you can just use a subtype in place of the type.
*  2. invariant.
*      if the type requires the same type.
*  3. contravariant.
*      this comes up for functions arguments.
*      the more useful the variable, the less useful the function
*/

#[cfg(test)]
mod tests {
    use super::*;

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
