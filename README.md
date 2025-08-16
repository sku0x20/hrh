# hrh — Heal Release Helper

hrh ("Heal Release Helper") is a small utility that reads a YAML declaration of a Helm release and executes Helm with the corresponding arguments.

- Input: a .yaml file containing Helm release information
- Action: invokes `helm upgrade --install ...` using the provided data

## Features
- Simple CLI: provide a YAML file and (optionally) a custom Helm binary path
- Resolves a relative `valuesFile` path relative to the YAML file’s directory
- Minimal debug logging toggled via `DEBUG=1`

## Installation
Build from source with Rust (stable):

```
cargo build --release
```

The binary will be at `target/release/hrh`.

## Usage
Basic usage:

```
hrh -f path/to/release.yaml
```

Use a custom Helm binary (e.g., a wrapper script):

```
hrh -f path/to/release.yaml --helm-path /usr/local/bin/helm
```

Enable debug logging:

```
DEBUG=1 hrh -f path/to/release.yaml
```

### Command-line flags
- `-f, --file <PATH>`: Path to the YAML declaration file (required)
- `--helm-path <PATH>`: Path to the Helm executable (default: `helm`)

## YAML declaration format
Fields are in camelCase:

```yaml
# release.yaml
releaseName: my-release
namespace: my-namespace
valuesFile: values.yaml        # if relative, it is resolved relative to this YAML file
chartName: my-chart
repo: myrepo                   # this is combined with chartName as: myrepo/my-chart
repoUrl: https://example.com/helm-charts  # currently parsed but not used by hrh
```

## What hrh runs
Given the YAML above (assuming release.yaml and values.yaml are in the same directory), hrh executes the equivalent of:

```
helm upgrade --install my-release myrepo/my-chart \
  --namespace my-namespace \
  --values /absolute/path/to/values.yaml
```

Notes:
- If `valuesFile` is relative, hrh resolves it relative to the directory of the YAML file you pass with `-f`.
- `repoUrl` is accepted in the YAML but not acted upon by hrh at the moment (e.g., hrh does not add or update Helm repos).

## Example
Example input (similar to tests/resources/vm-agent.yaml):

```yaml
releaseName: pod-collector
namespace: observability
valuesFile: pod-collector.yaml
chartName: victoria-metrics-agent
repo: vm
repoUrl: https://victoriametrics.github.io/helm-charts
```

This results in:

```
helm upgrade --install pod-collector vm/victoria-metrics-agent \
  --namespace observability \
  --values <resolved-path>/pod-collector.yaml
```

## Development
Run the E2E test harness provided in the repo:

```
./e2e.sh
```

This builds the project and runs a test that executes hrh against a sample YAML, using a `fake_helm.sh` script that captures the arguments hrh would pass to Helm.

## License
This project is licensed under the terms of the LICENSE file included in the repository.
