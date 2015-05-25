pub fn is_isbn(upc: &[u8]) -> bool {
    // Is the UPC code from "Bookland"
    // https://en.wikipedia.org/wiki/Bookland
    &upc[0..3] == &[9,7,8]
}

#[test]
fn test_is_isbn() {
    assert!(is_isbn(&[9,7,8]));
    assert!(!is_isbn(&[1,2,3]));
}

pub fn upc_to_isbn(upc: &[u8]) -> Vec<u8> {
    let mut isbn = &upc[3..12];
    let mut sum: u8 = 0;

    for i in 0..9 {
        let j = &isbn[i];
        sum = sum + (10u8 - i as u8) * *j;
    }
    sum = sum % 11;
    sum = 11 - sum;

    // Create the result Vec and
    // push the checksum bit
    let mut r = Vec::from(isbn);
    r.push(sum);

    return r;
}

#[test]
fn test_upc_to_isbn() {
    assert!(upc_to_isbn(&[9,7,8,0,5,9,6,0,0,4,1,0,1]) == &[0,5,9,6,0,0,4,1,0,9])
}
