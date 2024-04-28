# statsd monitor

Monitor the metrics being published with the statsd protocol.

## Contributing

### Setup

1. Install nix https://github.com/DeterminateSystems/nix-installer
2. Track nixpkgs and rust-overlay (TODO: pin these explicitly)

```bash
nix-channel --add https://nixos.org/channels/nixos-23.11 nixpkgs
nix-channel --add https://github.com/oxalica/rust-overlay/archive/master.tar.gz rust-overlay
nix-channel --update
```

3. Install direnv

```bash
nix profile install nixpkgs#direnv
nix profile install nixpkgs#nix-direnv
```
