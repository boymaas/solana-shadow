use solana_client::client_error::ClientError;
use thiserror::Error;
use tokio::sync::watch::error::SendError;
use tokio_tungstenite::tungstenite::Error as WsError;

use crate::account::AccountSnapshot;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Expected a program account")]
  NotAProgramAccount,

  #[error("Solana RPC error")]
  SolanaClientError(#[from] ClientError),

  #[error("WebSocket error")]
  WebSocketError(#[from] WsError),

  #[error("Internal synchronization error")]
  InternalSubscriptionError(#[from] SendError<AccountSnapshot>),
}

pub type Result<T> = std::result::Result<T, Error>;
