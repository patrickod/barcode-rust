pub fn is_isbn(upc: &[u8]) -> bool {
    &upc[0..3] == &[9,7,8]
}

#[test]
fn test_is_isbn() {
    assert!(is_isbn(&[9,7,8]));
    assert!(!is_isbn(&[1,2,3]));
}

// pub fn upc_to_isbn(upc: &[u8]) -> Vec<u8> {
//     let mut isbn = upc[3..9];
//     let mut sum = 0;
//
//     for i in &upc[0..9] {
//         let add = &upc[i];
//         sum += (10 - i) * add;
//     }
//     sum = sum % 11;
//     sum = 11 - sum;
//
//     let end = (match sum {
//         10 => "X",
//         11 => "0",
//         _ => ""
//     });
//
//     isbn.to_owned().append(end as u8)
// }
