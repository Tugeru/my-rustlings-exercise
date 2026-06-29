fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {} and last is {}", a.len(), a[0], a[a.len() - 1]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
