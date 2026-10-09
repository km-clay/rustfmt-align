{
  description = "rustfmt with column alignment for struct fields, match arms and let bindings";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default ];
    };

    # Pinned by rust-toolchain.toml; rustfmt uses #[feature(rustc_private)] and so needs
    # rustc-dev, and only ever builds against the exact nightly it was written for.
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

    rustPlatform = pkgs.makeRustPlatform {
      cargo = toolchain;
      rustc = toolchain;
    };

    rustfmt-aligned = rustPlatform.buildRustPackage {
      pname = "rustfmt-aligned";
      version = "1.11.0";

      src = self;
      cargoLock.lockFile = ./Cargo.lock;

      # Leaving CFG_RELEASE_CHANNEL unset makes `is_nightly_channel!()` true, which is what
      # lets the alignment options be set from a rustfmt.toml without --unstable-features.
      LIBRARY_PATH = "${toolchain}/lib";

      # The binary dynamically links librustc_driver and libLLVM out of the toolchain.
      postFixup = ''
        for b in $out/bin/*; do
          patchelf --add-rpath "${toolchain}/lib" "$b"
        done
      '';

      # The suite wants test fixtures and a writable cargo home; run it from the devShell.
      doCheck = false;

      meta = {
        description = "rustfmt plus struct field, match arm and let binding alignment";
        mainProgram = "rustfmt";
      };
    };
  in
  {
    packages = {
      default = rustfmt-aligned;
      rustfmt-aligned = rustfmt-aligned;
    };

    devShells.default = pkgs.mkShell {
      buildInputs = [ toolchain ];
      LD_LIBRARY_PATH = "${toolchain}/lib";
    };
  });
}
