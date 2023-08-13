use anchor_lang::prelude::*;

pub const USER: &[u8] = b"user";

#[account]
#[derive(Default)]
pub struct User {
    pub owner: Pubkey,
    pub username: [u8; 16], 
    pub account_type: u8, 
    pub total_deposits: u64, 
    pub total_withdraws: u64, 
    pub incoming_tx: u32,
    pub outgoing_tx: u32,
    pub num_friends: u32,
    pub num_contracts: u32,
    pub num_proposals: u32,
    pub num_groups: u32,
    pub preferred_token: Option<Pubkey>,
}

impl User {
    pub fn new(
        &mut self,
        owner: Pubkey,
        username: [u8; 16],
        account_type: u8,
    ) -> Result<()> {
        self.owner = owner;
        self.username = username;
        self.account_type = account_type;
        Ok(())
    }

    pub fn get_user_address(pubkey: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                USER,
                pubkey.as_ref(),
            ],
            &crate::ID,
        )
    }
    
    pub fn get_user_signer_seeds<'a>(
        pubkey: &'a Pubkey, 
        bump: &'a u8
    ) -> [&'a [u8]; 3] {
        [USER.as_ref(), pubkey.as_ref(), bytemuck::bytes_of(bump)]
    }

    pub fn set_username(&mut self, username: [u8; 16]) {
        self.username = username;
    }

    pub fn increment_outgoing_transactions(&mut self) {
        self.outgoing_tx = self.outgoing_tx.saturating_add(1);
    }

    pub fn increment_incoming_transactions(&mut self) {
        self.incoming_tx = self.incoming_tx.saturating_add(1);
    }
    
    pub fn increment_friends(&mut self) {
        self.num_friends = self.num_friends.saturating_add(1);
    }

    pub fn increment_contracts(&mut self) {
        self.num_contracts = self.num_contracts.saturating_add(1);
    }

    pub fn increment_withdrawals(&mut self) {
        self.total_withdraws = self.total_withdraws.saturating_add(1);
    }

    pub fn increment_proposals(&mut self) {
        self.num_proposals = self.num_proposals.saturating_add(1);
    }

    pub fn increment_groups(&mut self) {
        self.num_groups = self.num_groups.saturating_add(1);
    }

    pub fn set_preferred_token(&mut self, token: Pubkey) {
        self.preferred_token = Some(token);
    }

    pub fn disable_preferred_token(&mut self) {
        self.preferred_token = None;
    }
}

#[account]
pub struct Username {
    pub user_account: Pubkey,
    pub username: [u8; 16]
}

impl Username {
    pub fn new(
        &mut self,
        user_account: Pubkey,
        username: [u8; 16],
    ) -> Result<()> {
        self.user_account = user_account;
        self.username = username;
        Ok(())
    }
}

#[account]
pub struct Friend {
    pub friend_account: Pubkey,
    pub friend_username: [u8; 16],
    pub friend_account_type: u8
}

impl Friend {
    pub fn new(
        &mut self,
        friend_account: Pubkey,
        friend_username: [u8; 16],
        friend_account_type: u8
    ) -> Result<()> {
        self.friend_account = friend_account;
        self.friend_username = friend_username;
        self.friend_account_type = friend_account_type;
        Ok(())
    }
}
