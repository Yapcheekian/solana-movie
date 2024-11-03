use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
    Invalid,
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (instruction_byte, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        msg!("calliing instruction: {}", instruction_byte);

        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();

        let movie = match instruction_byte {
            1 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
            },
            _ => Self::Invalid,
        };

        Ok(movie)
    }
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String,
}
