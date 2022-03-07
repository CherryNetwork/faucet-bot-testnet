use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sp_keyring::AccountKeyring;
use subxt::sp_core::crypto::Ss58Codec;
use subxt::{sp_runtime::AccountId32, ClientBuilder, DefaultConfig, DefaultExtra};

#[subxt::subxt(runtime_metadata_path = "polkadot_metadata.scale")]
pub mod polkadot {}

#[command]
async fn claim(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let account_id = args.parse::<String>().unwrap();
    let mut iter = api.storage().system().account_iter(None).await?;
    // let mut target_acc;

    while let Some((key, account)) = iter.next().await? {
        // target_acc = AccountKeyring::from_account_id
        // if accound_id == target_acc {
        println!("{:?}: {:?}", hex::encode(key), account);
        // let signer = PairSigner::new(pot);
        let dest = AccountKeyring::from_account_id(AccountId32::from_string(&account_id)).unwrap();

        let hash = api
            .tx()
            .balances()
            .transfer(dest, 10_000)
            .sign_and_submit(&signer)
            .await?;

        println!("Balance transfer extrinscic submitted: {}", hash);

        // } else {
        //    println!("Account doesn't exist.");
        // }
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, "nice").await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
