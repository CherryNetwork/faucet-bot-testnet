use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use std::env;
use subxt::sp_core::crypto::Ss58Codec;
use subxt::sp_runtime::AccountId32;
use subxt::sp_runtime::MultiAddress;
use subxt::PairSigner;
use subxt::{ClientBuilder as SubstrateClientBuilder, DefaultConfig, DefaultExtra};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod metadata {}

#[command]
async fn claim(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let account_id = args.parse::<String>().unwrap();

    let api = SubstrateClientBuilder::new()
        .set_url(env::var("WS_URL").unwrap_or("wss://testnet-seeder.cherry.place:443".to_string()))
        .build()
        .await
        .unwrap()
        .to_runtime_api::<metadata::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let phrase = env::var("PHRASE")?;
    let pair = Pair::from_string(&phrase.to_string(), None).unwrap();
    let signer = PairSigner::<_, _, Pair>::new(pair);
    let dest = MultiAddress::from(AccountId32::from_string(&account_id).unwrap());

    let hash = api
        .tx()
        .balances()
        .transfer_keep_alive(dest, 5000000000000000000) // existential deposit - @charmitro
        .sign_and_submit(&signer)
        .await?;

    msg.react(&ctx.http, ReactionType::Unicode("🚀".to_string()))
       .await?;

    println!("Balance transfer extrinscic submitted: {}", hash);

    Ok(())
}
