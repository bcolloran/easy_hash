use easy_hash::EasyHash;
use std::cell::OnceCell;

#[test]
fn test_once_cell_hash_changes() {
    let cell: OnceCell<u32> = OnceCell::new();
    let empty_hash = cell.ehash();
    cell.set(5).unwrap();
    let filled_hash = cell.ehash();
    assert_ne!(empty_hash, filled_hash);
}

#[test]
fn test_different_cells_same_contents() {
    let cell_1: OnceCell<u32> = OnceCell::new();
    let cell_2: OnceCell<u32> = OnceCell::new();
    cell_1.set(10).unwrap();
    cell_2.set(10).unwrap();
    assert_eq!(cell_1.ehash(), cell_2.ehash());
}
