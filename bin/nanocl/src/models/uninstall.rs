use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct UninstallOpts {
  /// The docker host where nanocl is installed default is unix:///var/run/docker.sock
  #[clap(long)]
  pub(crate) docker_host: Option<String>,
  /// Uninstall template to use for nanocl by default it's detected
  #[clap(short, long)]
  pub(crate) template: Option<String>,
}