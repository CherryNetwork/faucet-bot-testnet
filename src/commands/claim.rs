use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sp_keyring::AccountKeyring;
use subxt::sp_core::crypto::Ss58Codec;
use subxt::sp_runtime::AccountId32;
use subxt::sp_runtime::MultiAddress;
use subxt::PairSigner;

use crate::SubstrateAPIContainer;

#[command]
async fn claim(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let api = {
        let data_read = ctx.data.read().await;

        data_read
            .get::<SubstrateAPIContainer>()
            .expect("could not get SubstrateAPIContainer")
            .clone()
    };

    let account_id = args.parse::<String>().unwrap();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = MultiAddress::from(AccountId32::from_string(&account_id).unwrap());

    let hash = api
        .tx()
        .balances()
        .transfer_keep_alive(dest, 10_0000000000000) // existential deposit - @charmitro
        .sign_and_submit(&signer)
        .await
        .unwrap();


    println!("Balance transfer extrinscic submitted: {}", hash);

    // TODO: Update the database - @charmitro


    Ok(())
}
