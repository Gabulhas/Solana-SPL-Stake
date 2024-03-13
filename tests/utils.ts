import {
  Transaction,
  SystemProgram,
  Keypair,
  Connection,
  PublicKey,
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createInitializeMintInstruction,
  getMinimumBalanceForRentExemptMint,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  createTransferInstruction,
} from "@solana/spl-token";

type MintConfig = {
  numDecimals: number;
  numberTokens: number;
};


const createNewMintTransaction = async (
  connection: Connection,
  payer: Keypair,
  mintKeypair: Keypair,
  destinationWallet: PublicKey,
  mintAuthority: PublicKey,
  freezeAuthority: PublicKey,
  numDecimals: number,
  numberTokens: number,
) => {
  //Get the minimum lamport balance to create a new account and avoid rent payments
  const requiredBalance = await getMinimumBalanceForRentExemptMint(connection);
  //metadata account associated with mint
  //get associated token account of your wallet
  const tokenATA = await getAssociatedTokenAddress(
    mintKeypair.publicKey,
    destinationWallet
  );

  return new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: mintKeypair.publicKey,
      space: MINT_SIZE,
      lamports: requiredBalance,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      mintKeypair.publicKey, //Mint Address
      numDecimals, //Number of Decimals of New mint
      mintAuthority, //Mint Authority
      freezeAuthority, //Freeze Authority
      TOKEN_PROGRAM_ID
    ),
    createAssociatedTokenAccountInstruction(
      payer.publicKey, //Payer
      tokenATA, //Associated token account
      payer.publicKey, //token owner
      mintKeypair.publicKey //Mint
    ),
    createMintToInstruction(
      mintKeypair.publicKey, //Mint
      tokenATA, //Destination Token Account
      mintAuthority, //Authority
      numberTokens * Math.pow(10, numDecimals) //number of tokens
    )
  );
};

const createAirdropToAccountTransaction = async (
  mint: PublicKey,
  mintOwner: Keypair,
  destinationWallet: PublicKey,
  amount: number
) => {
  const sourceATA = await getAssociatedTokenAddress(mint, mintOwner.publicKey);
  const destinationATA = await getAssociatedTokenAddress(
    mint,
    destinationWallet
  );

  return new Transaction().add(
    createAssociatedTokenAccountInstruction(
      mintOwner.publicKey, //Payer
      destinationATA, //Associated token account
      destinationWallet, //token owner
      mint //Mint
    ),
    createTransferInstruction(
      sourceATA,
      destinationATA,
      mintOwner.publicKey,
      amount,
      []
    )
  );
};

export {
  MintConfig,
  createNewMintTransaction,
  createAirdropToAccountTransaction
}

