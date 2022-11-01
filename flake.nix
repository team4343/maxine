{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = {
    self,
    nci,
    ...
  } @ inputs: let
    wpilib-cross-compiler = {
      config,
      lib,
      pkgs,
      ...
    }:
      pkgs.stdenv.mkDerivation rec {
        name = "wpilib-cross-compiler";
        version = "2022-1";

        nativeBuildInputs = with pkgs; [
          # Patch our binaries!
          autoPatchelfHook

          # Binary dependencies (patched during build)
          ncurses5.dev
          zlib.dev
          expat.dev
          xz.dev
          python27Full
          libclang.dev
        ];

        src = pkgs.fetchurl {
          url = "https://github.com/wpilibsuite/roborio-toolchain/releases/download/v2022-1/FRC-2022-Linux-Toolchain-7.3.0.tar.gz";
          sha256 = "sha256-snzeMC5G0RUkrt9mQSm8OsffAqeND55Ks/H+tA1merQ=";
        };
        sourceRoot = ".";

        installPhase = ''
          cp -r frc2022/roborio $out
        '';
      };
  in
    nci.lib.makeOutputs {
      root = ./.;

      config = common: {
        shell = {
          packages = with common.pkgs; [
            (pkgs.callPackage wpilib-cross-compiler {})
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
