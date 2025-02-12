use solana_client::client_error::ClientError;
use solana_sdk::pubkey::ParsePubkeyError;
use thiserror::Error;
use tokio::task::JoinError;
use tokio_tungstenite::tungstenite::Error as WsError;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Expected a program account")]
  NotAProgramAccount,

  #[error("Invalid Argument")]
  InvalidArguemt,

  #[error("Background worker is dead")]
  WorkerDead,

  #[error("Solana RPC error")]
  SolanaClientError(#[from] ClientError),

  #[error("WebSocket error")]
  WebSocketError(#[from] WsError),

  #[error("Notification for an unknown subscription")]
  UnknownSubscription,

  #[error("Unsupported RPC message format")]
  UnsupportedRpcFormat,

  #[error("Internal error")]
  InternalError,

  #[error("Invalid JSON-RPC message")]
  InvalidRpcMessage(#[from] serde_json::Error),

  #[error("Failed to parse public key")]
  InvalidPublicKey(#[from] ParsePubkeyError),

  #[error("Failed to parse base64 data")]
  InvalidBase64Data(#[from] base64::DecodeError),

  #[error("Internal synchronization error")]
  InternalSynchronizationError(#[from] JoinError),
}

pub type Result<T> = std::result::Result<T, Error>;
