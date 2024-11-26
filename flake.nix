{
  description = "Resolve DNS names to IPv4 or IPv6 addresses on the command line";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        # Toolchain for use in development shell
        toolchainFull = with fenix.packages.${system}; combine [
          complete.rustc
          complete.cargo
          complete.clippy
          complete.rustfmt
          targets.x86_64-unknown-linux-musl.latest.rust-std
        ];

        # Toolchain to build static binaries
        toolchainStatic = with fenix.packages.${system}; combine [
          minimal.rustc
          minimal.cargo
          targets.x86_64-unknown-linux-musl.latest.rust-std
        ];

        naersk' = pkgs.callPackage naersk {};

        naerskStatic = naersk.lib.${system}.override {
          cargo = toolchainStatic;
          rustc = toolchainStatic;
        };

        naerskDev = naersk.lib.${system}.override {
          cargo = toolchainFull;
          rustc = toolchainFull;
          clippy = toolchainFull;
        };

        staticPackage = naerskStatic.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkgsStatic.stdenv.cc
            pkgsStatic.openssl
          ];
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          # Tell Cargo to enable static compilation.
          # (https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags)
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
        };

        nativePackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            stdenv.cc
            openssl
          ];
        };

        devPackage = naerskDev.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            stdenv.cc
            openssl
          ];
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "dnsip";
          tag = "latest";
          config = {
            Entrypoint = "${staticPackage}/bin/dnsip";
          };
        };
      in rec {
        packages = rec {
          static = staticPackage;
          native = nativePackage;
          # For `nix run` or `nix run .`
          default = native;
          docker = dockerImage;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = with packages; [ devPackage ];
          buildInputs = with pkgs; [
            just
          ];
          shellHook = ''
            user_shell=$(getent passwd "$(whoami)" |cut -d: -f 7)

            exec "$user_shell"
          '';
        };
      }
    );
}
