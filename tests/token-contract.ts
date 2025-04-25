import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenContract } from "../target/types/token_contract";
import { TOKEN_PROGRAM_ID, MINT_SIZE, createAssociatedTokenAccountInstruction,getAssociatedTokenAddress,createInitializeMintInstruction } from "@solana/spl-token";
import { assert } from "chai";
describe("token-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const mintKey:anchor.web3.Keypair = anchor.web3.Keypair.generate()
  const program = anchor.workspace.tokenContract as Program<TokenContract>;
  let associatedTokenAccount = undefined
  it("Mint a token!", async () => {
    const key = anchor.AnchorProvider.env().wallet.publicKey;
    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);

    associatedTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey,key);

    const mintTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: key,
        newAccountPubkey:mintKey.publicKey,
        space:MINT_SIZE,
        programId:TOKEN_PROGRAM_ID,
        lamports,
      }
      ),
      createInitializeMintInstruction(mintKey.publicKey,0,key,key),
      createAssociatedTokenAccountInstruction(key,associatedTokenAccount,key,mintKey.publicKey)
    );
    const res = await anchor.AnchorProvider.env().sendAndConfirm(mintTx,[mintKey])

    console.log("Account: ",res);
    console.log("Mint Key: ",mintKey.publicKey.toString());
    console.log("User: ",key.toString())

    const tx = await program.methods.mintToken().accounts({
      mint: mintKey.publicKey,
      tokenAccount: associatedTokenAccount,
      payer: key,
    }).rpc();
    console.log("Your transaction signature", tx);
    const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount)).value.data

    console.log(minted)
    console.log("Amount NFT: ",minted.parsed.info.tokenAmount)
  });
});
