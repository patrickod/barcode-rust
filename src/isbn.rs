pub fn isbn_13_valid(isbn: &[u8]) -> bool {
    // Must at least be 13 characters
    if isbn.len() != 13 {
        return false;
    }

    let checksum = isbn[0..12].iter().enumerate().map(|(i, &n)| {
        if i % 2 == 0 {
            n
        } else {
            3 * n
        }
    }).fold(0, |acc, n| acc + n) % 10;

    return checksum == *(isbn.last().unwrap());
}

#[test]
fn test_isbn_13_valid() {
    assert!(isbn_13_valid(&[9,7,8,0,7,6,5,3,1,2,8,1,5]));
    assert!(!isbn_13_valid(&[9,7,8,0,7,6,5,3,1,2,8,1,6]));
}
