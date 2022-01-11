import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolanaClock } from '../target/types/solana_clock';
import { assert } from 'chai';

describe('solana-clock', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaClock as Program<SolanaClock>;
  
  const userKeypair = new anchor.web3.Keypair();

  const userName = 'my-username';

  it('should create user', async () => {
    // act
    await program.rpc.createUser({
      userName
    }, {
      accounts: {
        systemProgram: anchor.web3.SystemProgram.programId,
        authority: program.provider.wallet.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        user: userKeypair.publicKey
      },
      signers: [userKeypair]
    });
    // assert
    const userAccount = await program.account.user.fetch(userKeypair.publicKey);
    assert.ok(userAccount.authority.equals(program.provider.wallet.publicKey))
    assert.equal(userAccount.userName, userName);
    assert.ok(userAccount.createdAt.eq(userAccount.updatedAt));
  });

  it('should update user', async () => {
    // arrange
    const newUserName = 'my-new-username';
    // act
    await program.rpc.updateUser({
      userName: newUserName
    }, {
      accounts: {
        authority: program.provider.wallet.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        user: userKeypair.publicKey
      },
    });
    // assert
    const userAccount = await program.account.user.fetch(userKeypair.publicKey);
    assert.equal(userAccount.userName, newUserName);
    assert.ok(userAccount.createdAt.lte(userAccount.updatedAt));
  });
});
