
// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them


/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
// pub struct Matcher<T>(std::marker::PhantomData<T>);
pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(_matcher: fn(T) -> bool, _subs: S) -> Matcher<T> {
        Self {
            matcher: _matcher,
            subs: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
// pub struct Fizzy<T>(std::marker::PhantomData<T>);
pub struct Fizzy<T> {
    matcher_list: Vec<Matcher<T>>
}

impl<T: ToString + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matcher_list: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matcher_list.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        // todo!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        _iter.map(move |e| {
            let res: String = self
                .matcher_list
                .iter()
                .filter(|m| (m.matcher)(e.clone()))
                .map(|m| m.subs.clone())
                .collect();

            if res.is_empty() {
                e.to_string()
            }
            else {
                res
            }
        })
    }
}

use std::ops::Rem;

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Rem<Output = T> + From<u8> + PartialEq + ToString + Clone>() -> Fizzy<T> {
    let res: Fizzy<T> = Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"));
    res
}
