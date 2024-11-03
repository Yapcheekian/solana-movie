pub mod instruction;
pub mod state;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use state::MovieAccountState;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let result = instruction::MovieInstruction::unpack(instruction_data).unwrap();
    match result {
        instruction::MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => {
            let accounts_iter = &mut accounts.iter();

            let user = next_account_info(accounts_iter)?;
            let movie_account = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            msg!("deriving PDA");
            let seeds: &[&[u8]] = &[user.key.as_ref(), title.as_bytes().as_ref()];
            let (pda, bump) = Pubkey::find_program_address(seeds, program_id);

            msg!(
                "derived PDA: {} user provided the same is {} ({})",
                pda,
                pda == *movie_account.key,
                movie_account.key,
            );

            let storage_bytes = 1 + 1 + 4 + title.len() + 4 + description.len();

            msg!("calculating rent for {} bytes", storage_bytes);
            let rent = Rent::get()?;
            let required_lamports = rent.minimum_balance(storage_bytes);
            msg!("We'll need {} lamports for rent", required_lamports);

            msg!("Let's do a CPI on SytemProgram");

            let seeds_for_signing: &[&[&[u8]]] =
                &[&[user.key.as_ref(), title.as_bytes().as_ref(), &[bump]]];
            invoke_signed(
                &system_instruction::create_account(
                    user.key,
                    movie_account.key,
                    required_lamports,
                    storage_bytes.try_into().unwrap(),
                    program_id,
                ),
                &[user.clone(), movie_account.clone(), system_program.clone()],
                seeds_for_signing,
            );

            msg!("account created");

            msg!(
                "adding following movie: {}, {}, {}",
                title,
                rating,
                description
            );
        }
        instruction::MovieInstruction::Invalid => msg!("invalid"),
    }

    Ok(())
}
