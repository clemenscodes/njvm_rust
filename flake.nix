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
            ./src
            ./assets
            ./build.rs
            ./lib/bigint/Makefile
            ./lib/bigint/src/bigint.c
            ./lib/bigint/src/support.c
            ./lib/bigint/src/bigint.h
            ./lib/bigint/src/support.h
            ./lib/bigint/src/Makefile
            ./lib/bigint/tst/support.c
            ./lib/bigint/tst/testbip.c
            ./lib/bigint/tst/Makefile
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

          clippy = craneLib.cargoClippy (args
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          doc = craneLib.cargoDoc (args
            // {
              inherit cargoArtifacts;
            });

          fmt = craneLib.cargoFmt {
            inherit src;
          };

          audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          nextest = craneLib.cargoNextest (args
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });

          coverage = craneLib.cargoLlvmCov (args
            // {
              inherit cargoArtifacts;
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
