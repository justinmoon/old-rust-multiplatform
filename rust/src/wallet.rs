use std::sync::Arc;

use crate::cdk_integration_tests::{attempt_to_swap_pending, wait_for_mint_to_be_paid};
use anyhow::{bail, Result};
use bip39::Mnemonic;
use cdk::amount::SplitTarget;
use cdk::cdk_database::WalletMemoryDatabase;
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{
    CurrencyUnit, MeltBolt11Request, MeltQuoteState, MintBolt11Request, PreMintSecrets, Proofs,
    SecretKey, State, SwapRequest,
};
use cdk::wallet::client::{HttpClient, MintConnector};
use cdk::wallet::Wallet;
use cdk_fake_wallet::{create_fake_invoice, FakeInvoiceDescription};

const MINT_URL: &str = "http://127.0.0.1:8086";

// If both pay and check return pending input proofs should remain pending
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_fake_tokens_pending() -> Result<()> {
    let wallet = Wallet::new(
        MINT_URL,
        CurrencyUnit::Sat,
        Arc::new(WalletMemoryDatabase::default()),
        &Mnemonic::generate(12)?.to_seed_normalized(""),
        None,
    )?;

    let mint_quote = wallet.mint_quote(100.into(), None).await?;

    wait_for_mint_to_be_paid(&wallet, &mint_quote.id, 60).await?;

    let _mint_amount = wallet
        .mint(&mint_quote.id, SplitTarget::default(), None)
        .await?;

    let fake_description = FakeInvoiceDescription {
        pay_invoice_state: MeltQuoteState::Pending,
        check_payment_state: MeltQuoteState::Pending,
        pay_err: false,
        check_err: false,
    };

    let invoice = create_fake_invoice(1000, serde_json::to_string(&fake_description).unwrap());

    let melt_quote = wallet.melt_quote(invoice.to_string(), None).await?;

    let melt = wallet.melt(&melt_quote.id).await;

    assert!(melt.is_err());

    attempt_to_swap_pending(&wallet).await?;

    Ok(())
}
