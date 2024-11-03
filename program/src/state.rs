use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    is_initialized: u8,
    rating: u8,
    title: String,
    description: String,
}
