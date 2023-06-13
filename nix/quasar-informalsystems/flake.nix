{
  description = "A reproducible package set for Cosmos";

  inputs = {
    # Nix Inputs
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
    pre-commit-hooks.url = github:cachix/pre-commit-hooks.nix;
    sbt-derivation.url = github:zaninime/sbt-derivation;
    nix-std.url = github:chessai/nix-std;

    # Has to follow flake-utils in order to get aarch64-darwin
    # can revert after https://github.com/cachix/pre-commit-hooks.nix/pull/142
    pre-commit-hooks.inputs.flake-utils.follows = "flake-utils";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";

    # Freshautomations inputs
    stoml-src.url = github:freshautomations/stoml;
    stoml-src.flake = false;

    sconfig-src.url = github:freshautomations/sconfig;
    sconfig-src.flake = false;

    # Relayer Sources
    ibc-rs-src.url = github:informalsystems/ibc-rs/v1.0.0;
    ibc-rs-src.flake = false;

    relayer-src.url = github:cosmos/relayer/v1.0.0;
    relayer-src.flake = false;

    # Chain Sources

    gaia-main-src.flake = false;
    gaia-main-src.url = github:cosmos/gaia;

    ibc-go-v2-src.flake = false;
    ibc-go-v2-src.url = github:cosmos/ibc-go/v2.4.1;

    ibc-go-v3-src.flake = false;
    ibc-go-v3-src.url = github:cosmos/ibc-go/v3.3.0;

    ibc-go-v4-src.flake = false;
    ibc-go-v4-src.url = github:cosmos/ibc-go/v4.2.0;

    ibc-go-v5-src.flake = false;
    ibc-go-v5-src.url = github:cosmos/ibc-go/v5.1.0;

    ibc-go-v6-src.flake = false;
    ibc-go-v6-src.url = github:cosmos/ibc-go/v6.1.0;

    ibc-go-v7-src.flake = false;
    ibc-go-v7-src.url = github:cosmos/ibc-go/v7.0.0;

    cosmos-sdk-src.flake = false;
    cosmos-sdk-src.url = github:cosmos/cosmos-sdk/v0.46.0;

    osmosis-src.flake = false;
    osmosis-src.url = github:osmosis-labs/osmosis/v15.0.0;

    quasar-src.flake = false;
    quasar-src.url = git+ssh://git@github.com/quasar-finance/quasar.git;

    wasmvm_1-src.flake = false;
    wasmvm_1-src.url = github:CosmWasm/wasmvm/v1.0.0;

    wasmvm_1_2_3-src.flake = false;
    wasmvm_1_2_3-src.url = github:CosmWasm/wasmvm/v1.2.3;

    # for quasar which has wasmd v0.31.0
    wasmvm_1_2_0-src.flake = false;
    wasmvm_1_2_0-src.url = github:CosmWasm/wasmvm/v1.2.0;

    wasmvm_1_1_1-src.flake = false;
    wasmvm_1_1_1-src.url = github:CosmWasm/wasmvm/v1.1.1;

    wasmvm_1_beta7-src.flake = false;
    wasmvm_1_beta7-src.url = github:CosmWasm/wasmvm/v1.0.0-beta7;

    wasmvm_0_16_3-src.flake = false;
    wasmvm_0_16_3-src.url = github:CosmWasm/wasmvm/v0.16.3;
  };

  outputs = inputs:
    with inputs.flake-utils.lib;
      eachSystem
      [
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
      ]
      (system: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
            inputs.sbt-derivation.overlay
          ];
        };
        eval-pkgs = import inputs.nixpkgs {system = "x86_64-linux";};
        resources = (import ./resources.nix) {
          inherit inputs pkgs eval-pkgs system;
        };
        tests = (import ./tests.nix) {
          inherit (resources) packages;
          inherit pkgs system;
        };
      in rec {
        # nix build .#<app>
        packages = flattenTree (resources.packages // resources.devShells // tests);

        # nix flake check
        checks = (import ./checks.nix) {
          inherit inputs system;
          packages = resources.packages;
        };

        # nix develop
        devShells = resources.devShells;

        # nix run .#<app>
        apps = {
          # func
          stoml = mkApp {
            name = "stoml";
            drv = packages.stoml;
          };
          sconfig = mkApp {
            name = "sconfig";
            drv = packages.sconfig;
          };
          gm = mkApp {
            name = "gm";
            drv = packages.gm;
          };
          # chains
          gaia-main = mkApp {
            name = "gaia";
            drv = packages.gaia-main;
            exePath = "/bin/gaiad";
          };
          osmosis = mkApp {
            name = "osmosis";
            drv = packages.osmosis;
            exePath = "/bin/osmosisd";
          };
          quasar = mkApp {
            name = "quasar";
            drv = packages.quasar;
            exePath = "/bin/quasarnoded";
          };
          # relayers
          hermes = mkApp {
            name = "hermes";
            drv = packages.hermes;
          };
          relayer = mkApp {
            name = "relayer";
            drv = packages.relayer;
          };
        };
      });
}
