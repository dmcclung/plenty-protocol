const anchor = require("@project-serum/anchor");
const assert = require("assert");
const { SystemProgram } = anchor.web3;
const common = require("@project-serum/common");
const { TOKEN_PROGRAM_ID, Token } = require("@solana/spl-token");

describe("plenty", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Plenty;

  const payer = anchor.web3.Keypair.generate();
  const loanAccount = anchor.web3.Keypair.generate();

  let authority = null;

  it("airdrops to payer", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 10000000000),
      "confirmed"
    );
  });

  it("initializes the system state", async () => {
    const [stateAddress, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("state_v1"))],
      program.programId
    );

    const [authorityAddress, nonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("authority_v1"))],
        program.programId
      );
    authority = authorityAddress;

    await program.rpc.init(bump, nonce, {
      accounts: {
        state: stateAddress,
        authority: authorityAddress,
        payer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
    });

    const _state = await program.account.state.fetch(stateAddress);
    assert.ok(_state.authority.equals(authorityAddress));
    assert.ok(_state.nonce === nonce);
    assert.ok(_state.bump === bump);
  });

  it("creates a new loan", async () => {
    const amount = 50000;

    const longTokenMint = await Token.createMint(
      provider.connection,
      payer,
      authority,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    await program.rpc.createLoan(new anchor.BN(amount), {
      accounts: {
        loan: loanAccount.publicKey,
        longTokenMint: longTokenMint.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [loanAccount],
    });

    const _loan = await program.account.loan.fetch(loanAccount.publicKey);
    assert.ok(_loan.user.equals(provider.wallet.publicKey));
    assert.ok(_loan.longTokenMint.equals(longTokenMint.publicKey));
    assert.ok(_loan.amount.toNumber() === amount);
  });

  it("trades long on a loan", async () => {
    const size = 1000;
    const [stateAddress, _bump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("state_v1"))],
        program.programId
      );

    // Fetch the loan and its state.
    const loan = await program.account.loan.fetch(loanAccount.publicKey);

    // The user his long token account.
    const userTokenAccount = await common.createTokenAccount(
      provider,
      loan.longTokenMint,
      provider.wallet.publicKey
    );

    await program.rpc.tradeLong(new anchor.BN(size), {
      accounts: {
        state: stateAddress,
        authority: authority,
        loan: loanAccount.publicKey,
        user: provider.wallet.publicKey,
        userTokenAccount: userTokenAccount,
        mint: loan.longTokenMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    });
  });
});
