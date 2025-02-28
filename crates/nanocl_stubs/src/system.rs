use bollard_next::service::SystemInfo;
use bollard_next::container::ListContainersOptions;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::config::DaemonConfig;

use super::cargo::CargoInspect;
use super::resource::Resource;

/// HostInfo contains information about the host and the docker daemon
#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct HostInfo {
  /// Docker contains information about the docker daemon
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub docker: SystemInfo,
  /// HostGateway is the gateway address of the host
  pub host_gateway: String,
  /// Daemon configuration
  pub config: DaemonConfig,
}

/// Version contain details about the current version nanocl
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Version {
  pub arch: String,
  pub channel: String,
  pub version: String,
  pub commit_id: String,
}

/// Event is a message sent by nanocld to connected clients
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum Event {
  /// NamespaceCreated is sent when a namespace is created
  NamespaceCreated(String),
  /// NamespaceDeleted is sent when a namespace is deleted
  CargoCreated(Box<CargoInspect>),
  /// CargoDeleted is sent when a cargo is deleted
  CargoDeleted(Box<CargoInspect>),
  /// CargoStarted is sent when a cargo is started
  CargoStarted(Box<CargoInspect>),
  /// CargoStopped is sent when a cargo is stopped
  CargoStopped(Box<CargoInspect>),
  /// CargoPatched is sent when a cargo is patched
  CargoPatched(Box<CargoInspect>),
  /// ResourceCreated is sent when a resource is created
  ResourceCreated(Box<Resource>),
  /// ResourceDeleted is sent when a resource is deleted
  ResourceDeleted(Box<Resource>),
  /// ResourcePatched is sent when a resource is patched
  ResourcePatched(Box<Resource>),
}

impl std::fmt::Display for Event {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Event::NamespaceCreated(key) => write!(f, "NamespaceCreated({key})"),
      Event::CargoCreated(cargo) => write!(f, "CargoCreated({})", cargo.key),
      Event::CargoDeleted(cargo) => write!(f, "CargoDeleted({})", cargo.key),
      Event::CargoStarted(cargo) => write!(f, "CargoStarted({})", cargo.key),
      Event::CargoStopped(cargo) => write!(f, "CargoStopped({})", cargo.key),
      Event::CargoPatched(cargo) => write!(f, "CargoPatched({})", cargo.key),
      Event::ResourceCreated(resource) => {
        write!(f, "ResourceCreated({})", resource.name)
      }
      Event::ResourceDeleted(resource) => {
        write!(f, "ResourceDeleted({})", resource.name)
      }
      Event::ResourcePatched(resource) => {
        write!(f, "ResourcePatched({})", resource.name)
      }
    }
  }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ProccessQuery {
  /// Return container from all nodes
  pub all: bool,
  /// Return this number of most recently created containers
  pub last: Option<isize>,
  /// Return the size of container as fields `SizeRw` and `SizeRootFs`
  pub size: bool,
  /// Show all containers running for the given namespace
  pub namespace: Option<String>,
}

impl From<ProccessQuery> for ListContainersOptions<String> {
  fn from(query: ProccessQuery) -> Self {
    ListContainersOptions {
      all: query.all,
      limit: query.last,
      size: query.size,
      ..Default::default()
    }
  }
}
