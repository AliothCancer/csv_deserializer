// TEST AS EXTERNAL USER
// No access to internal private code

mod dataset {
    mod cell_wrapper;
    mod columns;
    mod rows;
}

// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(add(2, 2), 4);
    }
}
