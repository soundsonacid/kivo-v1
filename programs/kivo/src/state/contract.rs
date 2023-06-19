use anchor_lang::{ prelude::*, AnchorDeserialize };
use std::convert::TryFrom;

pub const CONTRACT: &[u8] = b"contract";
pub const OBLIGOR: &[u8] = b"obligor";

#[account]
#[derive(Debug)]
pub struct Contract {
    pub sender: Pubkey,
    pub sender_token_account: Pubkey,
    pub receiver: Pubkey,
    pub receiver_token_account: Pubkey,
    pub amount: u64,
    pub schedule: String,
    pub active: bool,
    pub id: u64,
    pub bump: u8
}

impl Contract {
    pub fn new(
        &mut self,
        sender: Pubkey,
        sender_token_account: Pubkey,
        receiver: Pubkey,
        receiver_token_account: Pubkey,
        amount: u64,
        schedule: String,
        id: u64,
        bump: u8
    ) -> Result<()> {
        self.sender = sender;
        self.sender_token_account = sender_token_account;
        self.receiver = receiver;
        self.receiver_token_account = receiver_token_account;
        self.amount = amount;
        self.schedule = schedule;
        self.active = false;
        self.id = id;
        self.bump = bump;
        Ok(())
    }

    pub fn enter(&mut self) {
        self.active = true;
    }

    pub fn reject(&mut self) {
        
    }

    pub fn get_contract_address(receiver: Pubkey, id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                CONTRACT,
                receiver.as_ref(),
                &id.to_be_bytes(),
            ],
            &crate::ID,
        )
    }
}

impl TryFrom<Vec<u8>> for Contract {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Contract::try_deserialize(&mut data.as_slice())
    }
}

#[account]
#[derive(Debug)]
pub struct Obligor {
    pub obligor: Pubkey,
    pub contract: Pubkey,
    pub active: bool,
    pub last_payment_at: Option<i64>,
    pub bump: u8,
}

impl Obligor {
    pub fn new(
        &mut self,
        obligor: Pubkey,
        contract: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.obligor = obligor;
        self.contract = contract;
        self.active = true;
        self.last_payment_at = None;
        self.bump = bump;

        Ok(())
    }

    pub fn get_obligor_address(obligor: Pubkey, contract: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                OBLIGOR,
                obligor.as_ref(),
                contract.as_ref()
            ],
            &crate::ID,
        )
    }
}

impl TryFrom<Vec<u8>> for Obligor {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Obligor::try_deserialize(&mut data.as_slice())
    }
}