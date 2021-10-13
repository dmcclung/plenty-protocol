import React from 'react'
import { useWallet } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js'

const programId = new PublicKey('E99aANyBFXYVRsWnNsu9HoMDy9HChuSyvmFUj4Z3Qyq7')

export const CreateLoan = (setStateAddress, setAuthorityAddress) => {
    const createLoan = async () => {
        const [authorityAddress, nonce] = await PublicKey.findProgramAddress(
            [Uint8Array.from("authority_v1")],
            programId
        )

        setAuthorityAddress(authorityAddress)
                
        const [stateAddress, bump] = await PublicKey.findProgramAddress(
            [Uint8Array.from("state_v1")],
            programId
        )

        setStateAddress(stateAddress)
        
        const wallet = useWallet()

        // TODO: This shouldn't be here on the client
        const loanAccount = anchor.web3.Keypair.generate()
        setLoanAccount(loanAccount)

        // TODO: not sure how to do this
        await program.rpc.init(bump, nonce, {
            accounts: {
            state: stateAddress,
            authority: authorityAddress,
            payer: wallet.publicKey,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            },
        });

        // TODO: This shouldn't be here either
        const longTokenMint = anchor.web3.Keypair.generate();
        const shortTokenMint = anchor.web3.Keypair.generate();

        await program.rpc.createLoan({
            accounts: {
                loan: loanAccount.publicKey,
                longTokenMint: longTokenMint.publicKey,
                shortTokenMint: shortTokenMint.publicKey,
                authority: authority,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                tokenProgram: TOKEN_PROGRAM_ID,
            },
            signers: [loanAccount, shortTokenMint, longTokenMint],
        });

        // TODO: Might need to set state here with these values for trade
        const _loan = await program.account.loan.fetch(loanAccount.publicKey);        
    }

    return (
        <div>
            <button onClick={ () => createLoan() }>Create Loan</button>
        </div>
    )
}