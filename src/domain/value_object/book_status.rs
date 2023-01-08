#[derive(Debug, PartialEq, Clone)]
pub enum BookStatus {
    AvailableForBorrow,
    Borrowing,
}