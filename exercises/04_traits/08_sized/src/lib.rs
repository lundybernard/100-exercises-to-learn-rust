pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sized_string_slice() {
        assert_eq!(std::mem::size_of::<&str>(), 16);
    }
}
