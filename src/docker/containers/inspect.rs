use crate::{http_client::HttpClient, utils::empty_string_as_none, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A request to remove an existing docker container
///
/// # Example
///
/// ```no_run
/// use longshoreman::{Docker, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let id = "CONTAINER_ID";
///
///     let response = Docker::new()
///         .containers()
///         .inspect(id)
///         .send()
///         .await?;
///
///     println!("{:#?}", response);
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Inspect<'a> {
    http_client: &'a HttpClient,
    container: &'a str,
    query: Query,
}

impl<'a> Inspect<'a> {
    pub(crate) fn new(http_client: &'a HttpClient, container: &'a str) -> Self {
        let query = Query::default();
        Self {
            http_client,
            container,
            query,
        }
    }

    /// Whether to return the size of the container as fields `size_rw` and
    /// `size_root_fs`.
    ///
    /// default is 'false'.
    #[must_use]
    pub fn size(mut self, size: bool) -> Self {
        self.query.size = size;
        self
    }

    /// Consume the request and return details about the container
    pub async fn send(self) -> Result<Response> {
        let endpoint = format!("/containers/{}/json", self.container);
        self.http_client
            .get(endpoint)
            .query(self.query)
            .into_json()
            .await
    }
}

#[derive(Debug, Default, Serialize)]
struct Query {
    size: bool,
}

/// A struct representation the information returned by a 'container inspect'
/// command
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    /// The security profile of the AppArmor module
    #[serde(deserialize_with = "empty_string_as_none")]
    pub app_armor_profile: Option<String>,

    /// The datetime that the container was created
    pub created: DateTime<Utc>,

    /// The driver runnning the container
    pub driver: String,

    /// The path on the host to the file containing the container hostname
    pub hostname_path: PathBuf,

    /// The path on the host to the file containing the container hosts
    pub hosts_path: PathBuf,

    /// The path on the host to the file containing the container logs
    pub log_path: PathBuf,

    /// The unique id of the container
    pub id: String,

    /// The unique id of the image which the container is an instance of
    pub image: String,

    /// The human-readable name of the container
    pub name: String,

    /// The number of times the container has been restarted
    pub restart_count: u16,

    /// The current state of the container
    pub state: State,
}

/// The state of a docker container
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    /// Container error
    #[serde(deserialize_with = "empty_string_as_none")]
    pub error: Option<String>,

    /// Exit code for the container
    pub exit_code: i32,

    /// The timestamp at thich the container finished
    pub finished_at: DateTime<Utc>,

    /// True if the container is 'dead'
    pub dead: bool,

    /// True if the container is 'paused'
    pub paused: bool,

    /// The PID of the main container process
    pub pid: u32,

    /// True if the container is 'restarting'
    pub restarting: bool,

    /// True if the container is 'running'
    pub running: bool,

    /// The timsetamp at which the container started
    pub started_at: DateTime<Utc>,

    /// The current status of the container
    pub status: Status,
}

/// The state of a docker container
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The container is dead
    Dead,

    /// The container is paused
    Paused,

    /// The container is in the process of restarting
    Restarting,

    /// The container is running normally
    Running,

    /// The container has been created, but never started
    Created,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialise_response() {
        let _: Response = serde_json::from_str(
        r#"{
            "AppArmorProfile": "",
            "Args": [
              "-c",
              "exit 9"
            ],
            "Config": {
              "AttachStderr": true,
              "AttachStdin": false,
              "AttachStdout": true,
              "Cmd": [
                "/bin/sh",
                "-c",
                "exit 9"
              ],
              "Domainname": "",
              "Env": [
                "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
              ],
              "Healthcheck": {
                "Test": [
                  "CMD-SHELL",
                  "exit 0"
                ]
              },
              "Hostname": "ba033ac44011",
              "Image": "ubuntu",
              "Labels": {
                "com.example.vendor": "Acme",
                "com.example.license": "GPL",
                "com.example.version": "1.0"
              },
              "MacAddress": "",
              "NetworkDisabled": false,
              "OpenStdin": false,
              "StdinOnce": false,
              "Tty": false,
              "User": "",
              "Volumes": {
                "/volumes/data": {}
              },
              "WorkingDir": "",
              "StopSignal": "SIGTERM",
              "StopTimeout": 10
            },
            "Created": "2015-01-06T15:47:31.485331387Z",
            "Driver": "devicemapper",
            "ExecIDs": [
              "b35395de42bc8abd327f9dd65d913b9ba28c74d2f0734eeeae84fa1c616a0fca",
              "3fc1232e5cd20c8de182ed81178503dc6437f4e7ef12b52cc5e8de020652f1c4"
            ],
            "HostConfig": {
              "MaximumIOps": 0,
              "MaximumIOBps": 0,
              "BlkioWeight": 0,
              "BlkioWeightDevice": [
                {}
              ],
              "BlkioDeviceReadBps": [
                {}
              ],
              "BlkioDeviceWriteBps": [
                {}
              ],
              "BlkioDeviceReadIOps": [
                {}
              ],
              "BlkioDeviceWriteIOps": [
                {}
              ],
              "ContainerIDFile": "",
              "CpusetCpus": "",
              "CpusetMems": "",
              "CpuPercent": 80,
              "CpuShares": 0,
              "CpuPeriod": 100000,
              "CpuRealtimePeriod": 1000000,
              "CpuRealtimeRuntime": 10000,
              "Devices": [],
              "IpcMode": "",
              "LxcConf": [],
              "Memory": 0,
              "MemorySwap": 0,
              "MemoryReservation": 0,
              "KernelMemory": 0,
              "OomKillDisable": false,
              "OomScoreAdj": 500,
              "NetworkMode": "bridge",
              "PidMode": "",
              "PortBindings": {},
              "Privileged": false,
              "ReadonlyRootfs": false,
              "PublishAllPorts": false,
              "RestartPolicy": {
                "MaximumRetryCount": 2,
                "Name": "on-failure"
              },
              "LogConfig": {
                "Type": "json-file"
              },
              "Sysctls": {
                "net.ipv4.ip_forward": "1"
              },
              "Ulimits": [
                {}
              ],
              "VolumeDriver": "",
              "ShmSize": 67108864
            },
            "HostnamePath": "/var/lib/docker/containers/ba033ac4401106a3b513bc9d639eee123ad78ca3616b921167cd74b20e25ed39/hostname",
            "HostsPath": "/var/lib/docker/containers/ba033ac4401106a3b513bc9d639eee123ad78ca3616b921167cd74b20e25ed39/hosts",
            "LogPath": "/var/lib/docker/containers/1eb5fabf5a03807136561b3c00adcd2992b535d624d5e18b6cdc6a6844d9767b/1eb5fabf5a03807136561b3c00adcd2992b535d624d5e18b6cdc6a6844d9767b-json.log",
            "Id": "ba033ac4401106a3b513bc9d639eee123ad78ca3616b921167cd74b20e25ed39",
            "Image": "04c5d3b7b0656168630d3ba35d8889bd0e9caafcaeb3004d2bfbc47e7c5d35d2",
            "MountLabel": "",
            "Name": "/boring_euclid",
            "NetworkSettings": {
              "Bridge": "",
              "SandboxID": "",
              "HairpinMode": false,
              "LinkLocalIPv6Address": "",
              "LinkLocalIPv6PrefixLen": 0,
              "SandboxKey": "",
              "EndpointID": "",
              "Gateway": "",
              "GlobalIPv6Address": "",
              "GlobalIPv6PrefixLen": 0,
              "IPAddress": "",
              "IPPrefixLen": 0,
              "IPv6Gateway": "",
              "MacAddress": "",
              "Networks": {
                "bridge": {
                  "NetworkID": "7ea29fc1412292a2d7bba362f9253545fecdfa8ce9a6e37dd10ba8bee7129812",
                  "EndpointID": "7587b82f0dada3656fda26588aee72630c6fab1536d36e394b2bfbcf898c971d",
                  "Gateway": "172.17.0.1",
                  "IPAddress": "172.17.0.2",
                  "IPPrefixLen": 16,
                  "IPv6Gateway": "",
                  "GlobalIPv6Address": "",
                  "GlobalIPv6PrefixLen": 0,
                  "MacAddress": "02:42:ac:12:00:02"
                }
              }
            },
            "Path": "/bin/sh",
            "ProcessLabel": "",
            "ResolvConfPath": "/var/lib/docker/containers/ba033ac4401106a3b513bc9d639eee123ad78ca3616b921167cd74b20e25ed39/resolv.conf",
            "RestartCount": 1,
            "State": {
              "Error": "",
              "ExitCode": 9,
              "FinishedAt": "2015-01-06T15:47:32.080254511Z",
              "Health": {
                "Status": "healthy",
                "FailingStreak": 0,
                "Log": [
                  {
                    "Start": "2019-12-22T10:59:05.6385933Z",
                    "End": "2019-12-22T10:59:05.8078452Z",
                    "ExitCode": 0,
                    "Output": ""
                  }
                ]
              },
              "OOMKilled": false,
              "Dead": false,
              "Paused": false,
              "Pid": 0,
              "Restarting": false,
              "Running": true,
              "StartedAt": "2015-01-06T15:47:32.072697474Z",
              "Status": "running"
            },
            "Mounts": [
              {
                "Name": "fac362...80535",
                "Source": "/data",
                "Destination": "/data",
                "Driver": "local",
                "Mode": "ro,Z", 
                "RW": false,
                "Propagation": ""
              }
            ]
          }"#
        ).unwrap();
    }
}
