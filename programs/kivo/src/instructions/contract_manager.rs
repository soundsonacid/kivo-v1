use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ system_program, sysvar };
use anchor_spl::token::*;
use anchor_spl::associated_token::*;
use std::mem::size_of;
use clockwork_sdk::state::Thread;
use clockwork_sdk::ThreadProgram;

use crate::state::contract::*;
use crate::state::user::User;

pub const USER: &[u8] = b"user";
pub const CONTRACT: &[u8] = b"contract";
pub const OBLIGOR: &[u8] = b"obligor";

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct ProposeContract<'info> {
    #[account(
        init, 
        payer = payer,
        space = 8 + size_of::<Contract>(),
        seeds = [
            CONTRACT,
            receiver_user_account.key().as_ref(),
            &id.to_be_bytes(),
            ],
        bump,
        )]
    pub contract: Box<Account<'info, Contract>>,

    #[account(mut)]
    pub sender_user_account: Box<Account<'info, User>>,

    #[account(associated_token::mint = mint, associated_token::authority = sender_user_account)]    
    pub sender_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        seeds = [
            USER, 
            payer.key().as_ref()
            ], 
        bump
        )]
    pub receiver_user_account: Box<Account<'info, User>>,

    #[account(associated_token::mint = mint, associated_token::authority = receiver_user_account)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct AcceptContract<'info> {
    #[account(address = Contract::get_contract_address(contract.receiver.key(), contract.id.clone()).0)]
    pub contract: Box<Account<'info, Contract>>,

    #[account(address = contract.receiver.key())]
    pub contract_owner: Box<Account<'info, User>>,

    #[account()]
    pub obligor_user_account: Box<Account<'info, User>>,

    #[account(
        init, 
        seeds = [
            OBLIGOR,
            payer.key().as_ref(),
            contract.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Obligor>(),
    )]
    pub obligor: Box<Account<'info, Obligor>>,

    #[account(associated_token::mint = mint, associated_token::authority = obligor.user_account)]    
    pub obligor_token_account: Box<Account<'info, TokenAccount>>, // this is the same as sender_token_account

    #[account(associated_token::mint = mint, associated_token::authority = contract.receiver)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(address = Thread::pubkey(contract.sender.key(), contract.id.to_le_bytes().to_vec()))]
    pub contract_thread: Box<Account<'info, Thread>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    // Add Thread Program ID
    pub thread_program: Program<'info, ThreadProgram>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
    
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RejectContract<'info> {
    #[account(address = Contract::get_contract_address(contract.receiver.key(), contract.id.clone()).0)]
    pub contract: Account<'info, Contract>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}