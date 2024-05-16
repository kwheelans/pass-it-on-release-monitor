# pass-it-on-release-monitor


## Configuration

### Example
```toml
[client]
key = "UVXu7wtbXHWNgAr6rWyPnaZbZK9aYin8"


[[client.interface]]
type = "http"
host = "localhost"
port = 8080

[monitor]
[[rancher-channel-server]]
url = "https://update.rke2.io/v1-release/channels"
channel = "stable"
notification = "rke2"

[[rancher-channel-server]]
url = "https://update.k3s.io/v1-release/channels"
channel = "stable"
notification = "k3s"

```
