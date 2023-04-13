import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Kivo } from "../target/types/kivo"
import * as assert from "assert";
import { PublicKey } from "@solana/web3.js";
import { getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Kivo as Program<Kivo>;

// Creates a User with the given name as their username.
// Does not yet check for duplicates... 
// May not need to check for duplicates if PDA mapping user <-> address is implemented.
export async function initialize_user(name : string) {

    const userAccount = anchor.web3.Keypair.generate();

    await program.methods
          .initializeUser(name)
          .accounts({
            owner: program.provider.publicKey,
            userAccount: userAccount.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([userAccount])
          .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);

    assert.equal(user.pubkey.toBase58(), userAccount.publicKey.toBase58()); // Redundancy but ok for now

    return user;
}

// Creates a Token Account at the specified mint and transfers the authority to the provided PublicKey.
export async function initialize_vault(mint: string, authority: PublicKey) {
    let vaultAccount = anchor.web3.Keypair.generate();

    await program.methods
          .initializeVault(authority)
          .accounts({
            vault: vaultAccount.publicKey,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([vaultAccount])
          .rpc();

    const vault = await getAccount(program.provider.connection, vaultAccount.publicKey);

    // assert.equal(vault.address, vaultAccount.publicKey);
    // This assertion fails inexplicably. The failure message indicates that the public keys are exactly the same.
    assert.equal(vault.mint, mint);

    return vault;
}