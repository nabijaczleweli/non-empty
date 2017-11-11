#![feature(try_from, type_ascription)]

use std::convert::TryFrom;

/// Non-empty list data type.
#[derive(Debug)]
pub struct NonEmpty<T>(T, Vec<T>);

impl<T: PartialEq> PartialEq for NonEmpty<T> {
    /// Equality comparison.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T: Default> Default for NonEmpty<T> {
    /// Default value.
    fn default() -> Self {
        NonEmpty(T::default(), Vec::new())
    }
}

impl<T> Into<Vec<T>> for NonEmpty<T> {
    /// Turns a non-empty list into a Vec.
    fn into(mut self) -> Vec<T> {
        let mut t = vec![self.0]; 
        t.append(&mut self.1);
        t
    }
}

/// Error arisen when given an empty vector.
#[derive(Debug, PartialEq)]
pub struct EmptyVecError;

impl<T> TryFrom<Vec<T>> for NonEmpty<T> {
    type Error = EmptyVecError;

    /// Turns a vec into a non-empty list.
    fn try_from(v: Vec<T>) -> Result<NonEmpty<T>, EmptyVecError> {
        if v.is_empty() {
            Err(EmptyVecError)
        } else {
            let mut it = v.into_iter();
            if let Some(n) = it.next() {
                Ok(NonEmpty(n, it.collect::<Vec<T>>()))
            } else {
                Err(EmptyVecError)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use NonEmpty;
    use EmptyVecError;
    use std::convert::TryFrom;

    #[test]
    fn i() {
        let i1 = NonEmpty(1, vec![2, 3]);
        assert_eq!(i1.into(): Vec<i32>, vec![1, 2, 3]);
    }

    #[test]
    fn f() {
        let f1 = vec![1, 2, 3];
        assert_eq!(NonEmpty::try_from(f1), Ok(NonEmpty(1, vec![2, 3])));

        let f2: Vec<i32> = vec![];
        assert_eq!(NonEmpty::try_from(f2), Err(EmptyVecError))
    }
}
