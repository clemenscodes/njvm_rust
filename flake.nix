{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    nix-filter = {
      url = "github:numtide/nix-filter";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-parts,
    crane,
    fenix,
    rust-overlay,
    advisory-db,
    nix-filter,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem = {system, ...}: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = nix-filter.lib {
          root = ./.;
          include = [
            ./Cargo.toml
            ./Cargo.lock
            ./rust-toolchain.toml
            ./rustfmt.toml
            ./.config
            ./crates/njvm/Cargo.toml
            ./crates/njvm/src
            ./crates/njvm/assets
            ./crates/cli/Cargo.toml
            ./crates/cli/src
            ./crates/bigint/Cargo.toml
            ./crates/bigint/build.rs
            ./crates/bigint/src
            ./crates/bigint/lib/bigint/Makefile
            ./crates/bigint/lib/bigint/src/bigint.c
            ./crates/bigint/lib/bigint/src/support.c
            ./crates/bigint/lib/bigint/src/bigint.h
            ./crates/bigint/lib/bigint/src/support.h
            ./crates/bigint/lib/bigint/src/Makefile
            ./crates/bigint/lib/bigint/tst/support.c
            ./crates/bigint/lib/bigint/tst/testbip.c
            ./crates/bigint/lib/bigint/tst/Makefile
          ];
        };

        inherit (craneLib.crateNameFromCargoToml {inherit src;}) pname version;

        args = {
          inherit src;
          strictDeps = true;
          nativeBuildInputs = [pkgs.rustPlatform.bindgenHook];
        };

        individualCrateArgs =
          args
          // {
            inherit cargoArtifacts version;
            doCheck = false;
          };

        cargoArtifacts = craneLib.buildDepsOnly args;

        njvm = craneLib.buildPackage (individualCrateArgs
          // {
            cargoExtraArgs = "-p ${pname}";
            inherit src;
          });
      in {
        checks = {
          inherit njvm;

          doc = craneLib.cargoDoc (args // {inherit cargoArtifacts;});
          fmt = craneLib.cargoFmt {inherit src;};
          audit = craneLib.cargoAudit {inherit src advisory-db;};
          coverage = craneLib.cargoLlvmCov (args // {inherit cargoArtifacts;});

          nextest = craneLib.cargoNextest (args
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });

          clippy = craneLib.cargoClippy (args
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });
        };

        packages = {
          inherit njvm;
          default = self.packages.${system}.njvm;
        };

        devShells = {
          default = craneLib.devShell {
            checks = self.checks.${system};
            nativeBuildInputs = [pkgs.rustPlatform.bindgenHook];
            packages = [
              pkgs.rust-analyzer
              pkgs.cargo-watch
              pkgs.cargo-llvm-cov
              pkgs.cargo-nextest
            ];
            RUST_SRC_PATH = "${craneLib.rustc}/lib/rustlib/src/rust/library";
            RUST_BACKTRACE = 1;
          };
        };

        formatter = pkgs.alejandra;
      };
    };
}
