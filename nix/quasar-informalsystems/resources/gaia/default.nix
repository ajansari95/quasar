{
  pkgs,
  inputs,
}: let
  mkCosmosGoApp =
    (import ../utilities.nix {
      inherit pkgs;
      inherit (inputs) nix-std;
    })
    .mkCosmosGoApp;
in
  with inputs;
    builtins.mapAttrs
    (_: mkCosmosGoApp)
    {
      gaia = {
        name = "gaia";
        vendorSha256 = "sha256-HAeDWNkA4Nghdmt4Od1T3mDlrupgh+7oZ88PeKFZ2z0=";
        version = "v9.1.1";
        src = gaia-src;
        engine = "tendermint/tendermint";

        # Test has to be skipped as end-to-end testing requires network access
        doCheck = false;
      };
    }
