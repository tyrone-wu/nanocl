use tabled::Tabled;
use clap::{Parser, Subcommand};

use nanocld_client::stubs::namespace::NamespaceSummary;

/// Namespace commands
#[derive(Debug, Subcommand)]
pub enum NamespaceCommands {
  /// Create new namespace
  Create(NamespaceOpts),
  /// Inspect a namespace
  Inspect(NamespaceOpts),
  /// Remove a namespace
  #[clap(alias("rm"))]
  Remove(NamespaceDeleteOpts),
  /// List existing namespaces
  #[clap(alias("ls"))]
  List(NamespaceListOpts),
}

#[derive(Debug, Parser)]
pub struct NamespaceListOpts {
  /// Show only namespace names
  #[clap(long, short)]
  pub quiet: bool,
}

#[derive(Debug, Parser)]
pub struct NamespaceDeleteOpts {
  /// skip confirmation
  #[clap(short = 'y')]
  pub skip_confirm: bool,
  /// list of namespace names to delete
  pub names: Vec<String>,
}

/// Manage namespaces
#[derive(Debug, Parser)]
#[clap(name = "nanocl namespace")]
pub struct NamespaceArgs {
  #[clap(subcommand)]
  pub commands: NamespaceCommands,
}

#[derive(Debug, Parser)]
pub struct NamespaceOpts {
  /// name of the namespace to create
  pub name: String,
}

#[derive(Tabled)]
pub struct NamespaceRow {
  pub(crate) name: String,
  pub(crate) cargoes: i64,
  pub(crate) instances: i64,
  pub(crate) gateway: String,
}

impl From<NamespaceSummary> for NamespaceRow {
  fn from(item: NamespaceSummary) -> Self {
    Self {
      name: item.name,
      cargoes: item.cargoes,
      instances: item.instances,
      gateway: item.gateway,
    }
  }
}
