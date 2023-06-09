{
  description = "Quasar Application";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.11";
    nixpkgsUnstable.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, nixpkgsUnstable, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        pkgsUnstable = import nixpkgsUnstable { inherit system; };
        srcDir = builtins.getCurrentDirectory;
      in rec {
        packages = rec {
          quasar = pkgs.stdenv.mkDerivation rec {
            name = "quasarnoded";
            src = ../../.;

            buildInputs = with pkgs; [
              cargo
              go_1_20
              gotools
              golangci-lint
              gopls
              go-outline
              gopkgs
              wget
              bash
              git
              jq
            ];

            buildPhase = ''
                set -e

                # This is not working cause not a .git directory
                #GIT_VERSION=$(git describe --tags --abbrev=0)
                #GIT_COMMIT=$(git rev-parse HEAD)
                GIT_VERSION=v0.1.0
                GIT_COMMIT=0ace32ab4b3a4bd6129f42fb23e496bdb0f24d23

                export GOPATH=$TEMPDIR/go
                export GOCACHE=$TEMPDIR/go-cache

                # Download go dependencies
                go mod download

                # Cosmwasm - Download correct libwasmvm version
                WASMVM_VERSION=$(curl --silent "https://api.github.com/repos/CosmWasm/wasmvm/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
                #wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm_muslc.$(uname -m).a -O $TEMPDIR/libwasmvm_muslc.a
                #Not sure about the following, check if flake-utils.lib.eachSystem is set properly
                if [[ $(uname -m) == "arm64" ]]; then
                    # We are on Mac Silicon
                    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm.aarch64.so -O $TEMPDIR/libwasmvm.aarch64.so
                else
                    # We are on a different platform
                    wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/libwasmvm_muslc.$(uname -m).a -O $TEMPDIR/libwasmvm_muslc.a
                fi

                # verify checksum
                #wget https://github.com/CosmWasm/wasmvm/releases/download/$WASMVM_VERSION/checksums.txt -O $TEMPDIR/checksums.txt
                #sha256sum $TEMPDIR/libwasmvm_muslc.a | grep $(cat $TEMPDIR/checksums.txt | grep $(uname -m) | cut -d ' ' -f 1)

                # Build quasarnoded binary
                # force it to use static lib (from above) not standard libgo_cosmwasm.so file
                # then log output of file /quasar/build/quasarnoded
                # then ensure static linking
                cd $src/cmd/quasarnoded
                GOWORK=off go build \
                    -mod=readonly \
                    -tags "netgo,ledger,muslc" \
                    -ldflags \
                        "-X github.com/cosmos/cosmos-sdk/version.Name="quasar" \
                        -X github.com/cosmos/cosmos-sdk/version.AppName="quasarnoded" \
                        -X github.com/cosmos/cosmos-sdk/version.Version=$GIT_VERSION \
                        -X github.com/cosmos/cosmos-sdk/version.Commit=$GIT_COMMIT \
                        -X github.com/cosmos/cosmos-sdk/version.BuildTags='netgo,ledger,muslc' \
                        -w -s -linkmode=external -extldflags '-Wl'" \
                        #-w -s -linkmode=external -extldflags '-Wl,-z,muldefs -static'" \
                    -trimpath \
                    -o $GOPATH/bin/quasarnoded . # This is wrong
            '';

            installPhase = ''
              set -e

              # Copy the binary to the bin directory
              install -D -t $out/bin $GOPATH/bin/quasarnoded

              # Copy the entrypoint and app_init scripts to the bin directory
              install -m755 ./tests/docker/bootstrap-scripts/entrypoint.sh $out/bin/
              install -m755 ./tests/docker/bootstrap-scripts/quasar_localnet.sh $out/bin/app_init.sh
            '';
          };

          quasarImage = pkgsUnstable.dockerTools.buildNixShellImage {
            name = "quasar";
            tag = "latest";
            drv = quasar.overrideAttrs (old: { src = null; });
          };
        };
        defaultPackage = packages.quasarImage;
      });
}
