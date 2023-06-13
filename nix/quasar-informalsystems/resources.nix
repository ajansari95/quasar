{
  system,
  inputs,
  pkgs,
  eval-pkgs,
}: let
  gaia-packages = import ./resources/gaia {
    inherit pkgs inputs;
  };

  ibc-packages = import ./resources/ibc-go {
    inherit pkgs inputs;
  };

  utilities = import ./resources/utilities.nix {
    inherit pkgs;
    inherit (inputs) nix-std;
  };

  scripts = import ./scripts {inherit pkgs;};

  cosmos-sdk-version = "v0.46.0";

  packages =
    rec {
      # Go packages
      stoml = pkgs.buildGoModule {
        name = "stoml";
        src = inputs.stoml-src;
        vendorSha256 = "sha256-37PcS7qVQ+IVZddcm+KbUjRjql7KIzynVGKpIHAk5GY=";
      };

      sconfig = pkgs.buildGoModule {
        name = "sconfig";
        src = inputs.sconfig-src;
        vendorSha256 = "sha256-ytpye6zEZC4oLrif8xe6Vr99lblule9HiAyZsSisIPg=";
      };

      quasar = utilities.mkCosmosGoApp {
        name = "quasar";
        version = "v0.1.0";
        src = inputs.quasar-src;
        vendorSha256 = "sha256-rxKwYS0+lKjY7EUvO8/1isCznf8vL4GN2b+ZKAhNpdM=";
        engine = "tendermint/tendermint";
        preFixup = ''
          ${utilities.wasmdPreFixupPhase libwasmvm_1_2_0 "quasarnoded"}
        '';
        buildInputs = [libwasmvm_1_2_0];
        proxyVendor = true;

        # Test has to be skipped as end-to-end testing requires network access
        doCheck = false;
      };

      osmosis = utilities.mkCosmosGoApp {
        name = "osmosis";
        version = "v15.0.0";
        src = inputs.osmosis-src;
        vendorSha256 = "sha256-4RNRAtQmWdi9ZYUH7Rn5VRef/ZhGB7WDwyelUf+U/rc=";
        tags = ["netgo"];
        engine = "tendermint/tendermint";
        preFixup = ''
          ${utilities.wasmdPreFixupPhase libwasmvm_1_1_1 "osmosisd"}
          ${utilities.wasmdPreFixupPhase libwasmvm_1_1_1 "chain"}
          ${utilities.wasmdPreFixupPhase libwasmvm_1_1_1 "node"}
        '';
        buildInputs = [libwasmvm_1_1_1];
        proxyVendor = true;

        # Test has to be skipped as end-to-end testing requires network access
        doCheck = false;
      };

      quicksilver = utilities.mkCosmosGoApp {
        name = "quicksilver";
        version = "v1.2.13";
        src = inputs.quicksilver-src;
        vendorSha256 = "sha256-DrDkTAlju+CoLdoEkdcFpV+iYVTej+Xw68m5cT3ghiQ=";
        engine = "tendermint/tendermint";
        preFixup = ''
          ${utilities.wasmdPreFixupPhase libwasmvm_1_1_0 "quicksilverd"}
        '';
        buildInputs = [libwasmvm_1_1_0];
        proxyVendor = true;

        # Test has to be skipped as end-to-end testing requires network access
        doCheck = false;
      };

      relayer = pkgs.buildGoModule {
        name = "relayer";
        src = inputs.relayer-src;
        vendorSha256 = "sha256-oJSxRUKXhjpDWk0bk7Q8r0AAc7UOhEOLj+SgsZsnzsk=";
        doCheck = false;
      };

      # Rust resources
      hermes = pkgs.rustPlatform.buildRustPackage {
        pname = "hermes";
        version = "v1.0.0";
        src = inputs.ibc-rs-src;
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        cargoSha256 = "sha256-0GZN3xq/5FC/jYXGVDIOrha+sB+Gv/6nzlFvpCAYO3M=";
        doCheck = false;
      };

      libwasmvm_1_2_3 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_1_2_3-src}/libwasmvm";
        version = "v1.2.3";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
          ln -s $out/lib/libwasmvm.so $out/lib/libwasmvm.${builtins.head (pkgs.lib.strings.splitString "-" system)}.so
        '';
        cargoSha256 = "sha256-+BaILTe+3qlI+/nz7Nub2hPKiDZlLdL58ckmiBxJtsk=";
        doCheck = false;
      };

      # quasar is using this as uses v0.31 of wamsd module
      libwasmvm_1_2_0 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_1_2_0-src}/libwasmvm";
        version = "v1.2.0";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
          ln -s $out/lib/libwasmvm.so $out/lib/libwasmvm.${builtins.head (pkgs.lib.strings.splitString "-" system)}.so
        '';
        cargoSha256 = "sha256-NPcR+gkdaSWhPfIrWI0BSwf7M4ktj3f/p5i80mu0NFA=";
        doCheck = false;
      };

      libwasmvm_1_1_1 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_1_1_1-src}/libwasmvm";
        version = "v1.1.1";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
          ln -s $out/lib/libwasmvm.so $out/lib/libwasmvm.${builtins.head (pkgs.lib.strings.splitString "-" system)}.so
        '';
        cargoSha256 = "sha256-97BhqI1FZyDbVrT5hdyEK7VPtpE9lQgWduc/siH6NqE";
        doCheck = false;
      };

      # quicksilver is using this as uses v0.29 of wasmd module
      libwasmvm_1_1_0 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_1_1_0-src}/libwasmvm";
        version = "v1.1.0";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
          ln -s $out/lib/libwasmvm.so $out/lib/libwasmvm.${builtins.head (pkgs.lib.strings.splitString "-" system)}.so
        '';
        cargoSha256 = "sha256-jkruBy5IfD+fhkE/72ceaevVT8ROjjnCwblscC/VtE0=";
        doCheck = false;
      };

      libwasmvm_1 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_1-src}/libwasmvm";
        version = "v1.0.0";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
          ln -s $out/lib/libwasmvm.so $out/lib/libwasmvm.${builtins.head (pkgs.lib.strings.splitString "-" system)}.so
        '';
        cargoSha256 = "sha256-Q8j9wESn2RBb05LcS7FiKGTPLgIPxWA0GZqHlTjkqpU=";
        doCheck = false;
      };

      libwasmvm_1beta7 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_1_beta7-src}/libwasmvm";
        version = "v1.0.0-beta7";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
        '';
        cargoSha256 = "sha256-G9wHl2JPgCDoMcykUAM0GrPUbMvSY5PbUzZ6G98rIO8=";
        doCheck = false;
      };

      libwasmvm_0_16_3 = pkgs.rustPlatform.buildRustPackage {
        pname = "libwasmvm";
        src = "${inputs.wasmvm_0_16_3-src}/libwasmvm";
        version = "v0.16.3";
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default];
        postInstall = ''
          cp ./bindings.h $out/lib/
        '';
        cargoSha256 = "sha256-MUTXxBCIYwCBCDNkFh+JrGMhKg20vC3wCGxqpZVa9Os=";
        doCheck = false;
      };

      # Misc
      gm = with pkgs;
        (import ./resources/gm) {
          inherit shellcheck lib makeWrapper gnused;
          inherit (inputs) ibc-rs-src;
          stoml = packages.stoml;
          sconfig = packages.sconfig;
          mkDerivation = stdenv.mkDerivation;
        };
#      ts-relayer =
#        ((import ./resources/ts-relayer) {
#          inherit pkgs eval-pkgs;
#          inherit (inputs) ts-relayer-src;
#        })
#        .ts-relayer;
#      ts-relayer-setup =
#        ((import ./resources/ts-relayer) {
#          inherit pkgs eval-pkgs;
#          inherit (inputs) ts-relayer-src;
#        })
#        .ts-relayer-setup;

      tx-database-migration = pkgs.writeTextFile {
        name = "tx_index_schema.sql";
        text = builtins.readFile ./fixtures/tx_index_schema.sql;
      };
    }
    // gaia-packages
    // ibc-packages;

  # Dev shells
  devShells = rec {
    default = nix-shell;
    nix-shell = pkgs.mkShell {
      shellHook = inputs.self.checks.${system}.pre-commit-check.shellHook;
      buildInputs = with pkgs;
        [
          rnix-lsp
          pass
          gnupg
          alejandra
          patchelf
          go
        ]
        ++ scripts;
    };

    cosmos-shell = pkgs.mkShell {
      buildInputs = with pkgs;
        [
          go
          rust-bin.stable.latest.default
          openssl
          shellcheck
        ]
        ++ builtins.attrValues packages;
    };

    osmosis-shell = pkgs.mkShell {
      buildInputs = with pkgs; [
        wget
        jq
        curl
        lz4
        python39
        packages.osmosis8
        packages.cosmovisor
      ];
      shellHook = ''
        export DAEMON_NAME=osmosisd
        export DAEMON_HOME=$HOME/.osmosisd
        export DAEMON_ALLOW_DOWNLOAD_BINARIES=false
        export DAEMON_LOG_BUFFER_SIZE=512
        export DAEMON_RESTART_AFTER_UPGRADE=true
        export UNSAFE_SKIP_BACKUP=true
      '';
    };
  };
in {inherit packages devShells;}
