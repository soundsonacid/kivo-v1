use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    pub sender_account: Pubkey, 
    pub mint: Pubkey,
    pub amount: u64,
    pub time_stamp: u64,
    pub receiver_account: Pubkey,
    pub receiver_transaction_account: Pubkey,
    pub status: bool 
}

impl Transaction {
    pub fn new(
        &mut self,
        sender_account: Pubkey,
        mint: Pubkey,
        amount: u64,
        time_stamp: u64,
        receiver_account: Pubkey,
        receiver_transaction_account: Pubkey,
        status: bool,
    ) -> Result<()> {
        self.sender_account = sender_account;
        self.mint = mint;
        self.amount = amount;
        self.time_stamp = time_stamp;
        self.receiver_account = receiver_account;
        self.receiver_transaction_account = receiver_transaction_account;
        self.status = status;
        Ok(())
    }

    pub fn fulfill(
        &mut self,
        fulfiller: Pubkey,
        requester: Pubkey,
        status: bool
    ) -> Result<()> {
        self.sender_account = fulfiller;
        self.receiver_account = requester;
        self.status = status;
        Ok(())
    }
}