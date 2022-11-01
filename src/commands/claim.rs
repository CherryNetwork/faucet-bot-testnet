use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::env;
use subxt::{
    ext::{
        sp_core::crypto::Ss58Codec,
        sp_runtime::{AccountId32, MultiAddress},
    },
    tx::PairSigner,
    OnlineClient, PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod cherry {}

#[command]
async fn claim(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let account_id = args.parse::<String>().unwrap();

    let api = OnlineClient::<PolkadotConfig>::from_url("wss://testnet-seeder.cherrynetwork.dev:443").await.unwrap();

    let phrase = env::var("PHRASE")?;
    let pair = Pair::from_string(&phrase.to_string(), None).unwrap();
    let signer = PairSigner::<_, _>::new(pair);
    let dest = MultiAddress::from(AccountId32::from_string(&account_id).unwrap());

    let tx = cherry::tx()
        .balances()
        .transfer_keep_alive(dest, 5000000000000000000); // existential deposit - @charmitro

    let hash = api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await
        .unwrap();

    msg.react(&ctx.http, ReactionType::Unicode("🚀".to_string()))
        .await?;

    println!("Balance transfer extrinscic submitted: {}", hash);

    Ok(())
}
