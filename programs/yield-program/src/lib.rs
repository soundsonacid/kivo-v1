use anchor_lang::prelude::*;

use crate::instructions::passive::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

pub mod state;
pub mod instructions;

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn handle_initialize_passive_lending_account(ctx: Context<InitializePassiveLendingAccount>, bump: u8) -> Result<()> {
        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let kivo_signer_seeds = &[&signature_seeds[..]];  

        let init_mfi_acc = marginfi::cpi::accounts::MarginfiAccountInitialize {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            authority: ctx.accounts.kivo_account.to_account_info(),
            fee_payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let init_mfi_acc_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            init_mfi_acc,
            kivo_signer_seeds
        );

        marginfi::cpi::marginfi_account_initialize(init_mfi_acc_ctx)?;

        ctx.accounts.passive_lending_account.new(
            ctx.accounts.kivo_account.key(),
            ctx.accounts.marginfi_account.key(),
            ctx.accounts.marginfi_group.key(),
        )?;

        Ok(())
    }

    pub fn handle_passive_lending_account_deposit(ctx: Context<PassiveLendingAccountDeposit>, amount: u64, bump: u8) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let kivo_signer_seeds = &[&signature_seeds[..]];  

        let mfi_deposit_acc = marginfi::cpi::accounts::LendingAccountDeposit {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            signer: ctx.accounts.kivo_account.to_account_info(),
            bank: ctx.accounts.marginfi_bank.to_account_info(),
            signer_token_account: ctx.accounts.kivo_token_account.to_account_info(),
            bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let mfi_deposit_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            mfi_deposit_acc,
            kivo_signer_seeds
        );

        marginfi::cpi::lending_account_deposit(mfi_deposit_ctx, amount)?;

        passive_lending_account.increment_deposits(amount);
        passive_lending_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_passive_lending_account_withdraw(ctx: Context<PassiveLendingAccountWithdraw>, amount: u64, bump: u8, withdraw_all:  Option<bool>) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let kivo_signer_seeds = &[&signature_seeds[..]];          

        let mfi_withdraw_acc = marginfi::cpi::accounts::LendingAccountWithdraw {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            signer: ctx.accounts.kivo_account.to_account_info(),
            bank: ctx.accounts.marginfi_bank.to_account_info(),
            destination_token_account: ctx.accounts.kivo_token_account.to_account_info(),
            bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
            bank_liquidity_vault_authority: ctx.accounts.bank_vault_authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let mfi_withdraw_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            mfi_withdraw_acc,
            kivo_signer_seeds,
        );
        
        let withdraw_all = withdraw_all;

        let amount = if withdraw_all == Some(true) {
          0
        } else {
          amount
        };

        marginfi::cpi::lending_account_withdraw(mfi_withdraw_ctx, amount, withdraw_all)?;

        passive_lending_account.increment_withdrawals(amount);
        passive_lending_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_passive_lending_account_borrow(ctx: Context<PassiveLendingAccountBorrow>, amount: u64, bump: u8) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let kivo_signer_seeds = &[&signature_seeds[..]];  

        let mfi_borrow_acc = marginfi::cpi::accounts::LendingAccountBorrow {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            signer: ctx.accounts.kivo_account.to_account_info(),
            bank: ctx.accounts.marginfi_bank.to_account_info(),
            destination_token_account: ctx.accounts.kivo_token_account.to_account_info(),
            bank_liquidity_vault_authority: ctx.accounts.bank_vault_authority.to_account_info(),
            bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let mfi_borrow_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            mfi_borrow_acc,
            kivo_signer_seeds,
        );

        marginfi::cpi::lending_account_borrow(mfi_borrow_ctx, amount)?;

        passive_lending_account.increment_borrows(amount);
        passive_lending_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_passive_lending_account_repay(ctx: Context<PassiveLendingAccountRepay>, amount: u64, repay_all: Option<bool>, bump: u8) -> Result<()> {
        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let kivo_signer_seeds = &[&signature_seeds[..]];

        let mfi_repay_acc = marginfi::cpi::accounts::LendingAccountRepay {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            signer: ctx.accounts.kivo_account.to_account_info(),
            bank: ctx.accounts.marginfi_bank.to_account_info(),
            signer_token_account: ctx.accounts.kivo_token_account.to_account_info(),
            bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let mfi_repay_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            mfi_repay_acc,
            kivo_signer_seeds,
        );

        let repay_all = repay_all;

        let amount = if repay_all == Some(true) {
          0
        } else {
          amount
        };

        marginfi::cpi::lending_account_repay(mfi_repay_ctx, amount, repay_all)?;

        Ok(())
    }

    pub fn handle_passive_lending_account_claim_interest(_ctx: Context<PassiveLendingAccountClaimInterest>, _amount: u64, _bump: u8) -> Result<()> {

        Ok(())
    }
}   

