import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorNftStaking } from "../target/types/anchor_nft_staking";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { KeypairSigner, PublicKey, createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount } from "@metaplex-foundation/umi";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("anchor-nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorNftStaking as Program<AnchorNftStaking>;

  // Create a Umi connection and payer
  const umi = createUmi(provider.connection);
  const payer = provider.wallet as NodeWallet;

  // Create a keypair for the NFT mint and collection mint
  let nftMint: KeypairSigner = generateSigner(umi);
  let collectionMint: KeypairSigner = generateSigner(umi);
  const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(nftMint.publicKey.toString());
  const collection: anchor.web3.PublicKey = new anchor.web3.PublicKey(collectionMint.publicKey.toString());

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(payer.payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  const configAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0];
  const rewardsMint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("rewards"), configAccount.toBuffer()], program.programId)[0];
  const userAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("user"), provider.publicKey.toBuffer()], program.programId)[0];
  const stakeAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stake"), mint.toBuffer(), configAccount.toBuffer()], program.programId)[0];


  it("Mint Collection NFT", async () => {
    // Create the collection NFT
    await createNft(umi, {
      mint: collectionMint,
      name: "My awesome collection",
      symbol: "YEY",
      uri: "https://example.com/collection.json",
      sellerFeeBasisPoints: percentAmount(5),
      collectionDetails: { 
        __kind: 'V1', size: 10,
      }
    }).sendAndConfirm(umi);
    console.log("Collection Mint: ", collectionMint.publicKey.toString());

  });

  it("Mint NFT", async () => {
    await createNft(umi, {
      mint: nftMint,
      name: "My super awesome NFT",
      symbol: "NFT",
      uri: "https://example.com/nft.json",
      sellerFeeBasisPoints: percentAmount(5.5),
      collection: {
        verified: true,
        key: collection,
      }
    }).sendAndConfirm(umi);
    console.log("NFT Mint: ", nftMint.publicKey.toString());
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


});
