{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = {
    self,
    nci,
    ...
  } @ inputs:
    nci.lib.makeOutputs {
      root = ./.;

      config = common: {
        shell = {
          packages =
            with common.pkgs; [
              pkg-config
              openssl.dev

              rust-analyzer

              cargo-outdated
              cargo-audit
              cargo-release
              cargo-tarpaulin
              cargo-nextest

              git-cliff
            ];
        };
      };
    };
}
