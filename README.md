# dnsip

dnsip is a simple command-line DNS client with modern features. It simply
prints ipaddresses so it's output is simple to use in scripts or one-off
commands.


## Getting Started

#### Nix

Run the flake directly:

`nix run github:koenw/dnsip`

Or install using an overlay:

```nix
inputs.dnsip.url "github:koenw/dnsip";

# [...]

{ config, pkgs, lib, ... }:
let
    pkgs = import nixpkgs rec { inherit system; overlays = [ dnsip.overlays.${system}.default ]; }
in {
	# [...]

	environment.systemPackages = [ pkgs.dnsip ];
}
```


#### Static (linux) binaries

The most straightforward way to get started might be to download the latest
static binaries from the [releases](https://github.com/koenw/dnsip/releases)
page.


#### docker

Alternatively, you could start dnsip in a container with `docker run
ghcr.io/koenw/dnsip`.


## Usage

```sh
❯ ./dnsip --help
dnsip 0.1.0
Resolve DNS names to IP addresses

dnsip resolves the given hostname and prints it's addresses to stdout, one per
line.

USAGE:
    dnsip [FLAGS] [OPTIONS] <host>

FLAGS:
        --edns0
            Use edns for larger records

    -h, --help
            Prints help information

        --preserve-intermediates
            Show intermediate responses

        --validate
            Use DNSSEC to validate the request

    -V, --version
            Prints version information


OPTIONS:
        --ip-strategy <ip-strategy>
             [default: ipv4andipv6]  [possible values: ipv4,
            ipv6, ipv4thenipv6, ipv6thenipv4,
            ipv4andipv6]
        --timeout <timeout>
            Timeout for the DNS request [default: 2s]

    -v <verbose>...
            Be more verbose [default: 0]


ARGS:
    <host>
            DNS name to resolve
```


## Development

Run `nix develop` to open a development shell.

| Command | Description |
| --- | --- |
| `nix develop` | Open a development shell |
| `nix build '.#static'` | Build a static (musl) binary |
| `nix build '.#native'` | Build a dynamically linked binary |
| `nix build '.#docker'` | Build the docker image |
