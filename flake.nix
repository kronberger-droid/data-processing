{
  description = "Data processing in Rust and Python, plotting with Typst";

  inputs = {
    # Locked to the same revs as the NixOS config's `flake.lock`, so the shell
    # reuses the store paths the system already has instead of fetching a
    # second package set. After a system update, re-sync by passing the
    # system's revs to `nix flake lock --override-input nixpkgs
    # github:NixOS/nixpkgs/<rev> --override-input rust-overlay
    # github:oxalica/rust-overlay/<rev>`. Take the rust-overlay rev from the
    # node `.nodes.root.inputs.rust-overlay` names (`rust-overlay_2`): the
    # plain `rust-overlay` node there is Rio's unfollowed copy.
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # The toolchain rides its own input so it is not welded to the nixpkgs pin.
    # Same pattern as the NixOS config's modules/home-manager/editors/dev-tools.nix.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    forAllSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];
    pkgsFor = system:
      import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
    # `default` bundles rustc/cargo/clippy/rustfmt; rust-analyzer resolves the
    # std sources through the toolchain's sysroot via rust-src.
    toolchainFor = pkgs:
      pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-analyzer" "rust-src"];
      };
  in {
    devShells = forAllSystems (system: let
      pkgs = pkgsFor system;
    in {
      default = pkgs.mkShell {
        nativeBuildInputs = [
          (toolchainFor pkgs)
          (pkgs.python3.withPackages (ps: [
            ps.numpy
            ps.scipy
            ps.pandas
            ps.matplotlib
          ]))
          pkgs.typst
          pkgs.tinymist
        ];
      };
    });
  };
}
