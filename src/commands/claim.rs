use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sp_keyring::AccountKeyring;
use subxt::sp_core::crypto::Ss58Codec;
use subxt::sp_runtime::AccountId32;
use subxt::sp_runtime::MultiAddress;
use subxt::PairSigner;

use crate::{db, SqliteLitePoolContaintainer, SubstrateAPIContainer};

#[command]
async fn claim(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let api = ctx
        .data
        .read()
        .await
        .get::<SubstrateAPIContainer>()
        .expect("could not get SqliteLitePoolContainer")
        .clone();

    let db_conn = ctx
        .data
        .read()
        .await
        .get::<SqliteLitePoolContaintainer>()
        .expect("could not get SqliteLitePoolContainer")
        .clone();

    let user_id = msg.author.id.to_string();

    let account_id = args.parse::<String>().unwrap();
    match db::add::add_user(&*db_conn, user_id, account_id.clone()).await {
        Err(err) => println!("{:?}", err),
        Ok(_) => {
            println!("passoed\n\n");
            let signer = PairSigner::new(AccountKeyring::Alice.pair());
            let dest = MultiAddress::from(AccountId32::from_string(&account_id).unwrap());

            let hash = api
                .tx()
                .balances()
                .transfer_keep_alive(dest, 10_0000000000000) // existential deposit - @charmitro
                .sign_and_submit(&signer)
                .await?;

            println!("Balance transfer extrinscic submitted: {}", hash);
        }
    }

    Ok(())
}
