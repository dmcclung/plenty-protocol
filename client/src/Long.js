
export const Long = () => {
    const size = 1000;
    const [stateAddress, _bump] =
    await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("state_v1"))],
        program.programId
    );

    // Fetch the loan and its state.
    const loan = await program.account.loan.fetch(loanAccount.publicKey);
    const userTokenAccount = anchor.web3.Keypair.generate();

    await program.rpc.tradeLong(new anchor.BN(size), {
      accounts: {
        state: stateAddress,
        authority: authority,
        loan: loanAccount.publicKey,
        user: provider.wallet.publicKey,
        userTokenAccount: userTokenAccount.publicKey,
        mint: loan.longTokenMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [userTokenAccount],
    });

    const _userTokenAccount = await common.getTokenAccount(
      provider,
      userTokenAccount.publicKey
    );
}


   