{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [
          (import rust-overlay)
          (self: super: {
            rust-toolchain = super.rust-bin.stable.latest.default;
          })
        ];
      in {
        devShells.default = pkgs.mkShellNoCC {
          packages = with pkgs; [ rust-toolchain cargo-edit cargo-nextest ];
          buildInputs = [ ];
          nativeBuildInputs = with pkgs;
            [ ] ++ lib.optionals stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [ SystemConfiguration ]);
        };
      });
}
