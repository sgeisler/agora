use http::uri::Authority;
use std::path::PathBuf;
use structopt::StructOpt;

const DEFAULT_PORT: &str = if cfg!(test) { "0" } else { "8080" };

#[derive(StructOpt)]
pub(crate) struct Arguments {
  #[structopt(long, default_value = "0.0.0.0", help = "Address to listen on")]
  pub(crate) address: String,
  #[structopt(long, default_value = "www", help = "Directory of files to serve")]
  pub(crate) directory: PathBuf,
  #[structopt(long, default_value = DEFAULT_PORT, help = "Port to listen on")]
  pub(crate) port: u16,
  #[structopt(
    long,
    help = "Host and port of the LND gRPC server. By default a locally running LND instance will expose its gRPC API on `localhost:10009`."
  )]
  pub(crate) lnd_rpc_authority: Option<Authority>,
  #[structopt(
    long,
    help = "Path to LND's TLS certificate. Needed if LND uses a self-signed certificate. By default LND writes its TLS certificate to `~/.lnd/tls.cert`.",
    requires = "lnd-rpc-authority"
  )]
  pub(crate) lnd_rpc_cert_path: Option<PathBuf>,
  #[structopt(
    long,
    help = "Path to an LND gRPC macaroon, that allows creating and querying invoices. Needed if LND requires macaroon authentication. By default LND writes its invoice macaroon to `~/.lnd/data/chain/bitcoin/mainnet/invoice.macaroon`.",
    requires = "lnd-rpc-authority"
  )]
  pub(crate) lnd_rpc_macaroon_path: Option<PathBuf>,
}
