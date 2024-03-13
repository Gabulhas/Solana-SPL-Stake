import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenStake } from "../target/types/token_stake";
import { createNewMintTransaction } from "./utils";

async function singleSeedFindPda(seed: string, programId: PublicKey): Promise<PublicKey> {
  const [pda] = await PublicKey.findProgramAddress(
    [Buffer.from(seed)],
    programId
  );
  return pda;
}

describe("token_stake", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenStake as Program<TokenStake>;

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  const mint_pair = anchor.web3.Keypair.generate();


  const platform_info_seed = program.idl.constants.find((c) => c.name === "PLATFORM_INFO_SEED").value
  const pool_info_seed = program.idl.constants.find((c) => c.name === "POOL_INFO_SEED").value
  const vault_info_seed = program.idl.constants.find((c) => c.name === "VAULT_INFO_SEED").value
  const user_info_seed = program.idl.constants.find((c) => c.name === "USER_INFO_SEED").value

  const platform_info_pda = singleSeedFindPda(platform_info_seed, program.programId)
  const pool_info_pda = singleSeedFindPda(pool_info_seed, program.programId)
  const vault_info_pda = singleSeedFindPda(vault_info_seed, program.programId)

  const total_to_mint =

    createNewMintTransaction()




  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

});
