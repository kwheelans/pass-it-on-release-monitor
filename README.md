# pass-it-on-release-monitor


## Configuration

### Example
```toml
[client]
key = "change me"

[[client.interface]]
type = "http"
host = "localhost"
port = 8080

[monitors]
[[monitors.monitor]]
type = "rancher-channel"
url = "https://update.rke2.io/v1-release/channels"
channel = "stable"
notification = "rke2"
frequency = 1
period = "day"

[[monitors.monitor]]
type = "rancher-channel"
url = "https://update.k3s.io/v1-release/channels"
channel = "stable"
notification = "k3s"


```
