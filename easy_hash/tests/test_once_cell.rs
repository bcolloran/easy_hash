use easy_hash::EasyHash;
use std::cell::OnceCell;

#[test]
fn test_once_cell_hash_changes() {
    let cell: OnceCell<u32> = OnceCell::new();
    let empty_hash = cell.ehash();
    cell.set(5).unwrap();
    let filled_hash = cell.ehash();
    assert_ne!(empty_hash, filled_hash);
    let other = OnceCell::new();
    other.set(5).unwrap();
    assert_eq!(filled_hash, other.ehash());
}
