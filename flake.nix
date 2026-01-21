{
  description = "Arceus monorepo development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Unified Rust toolchain for all projects (Tauri + backend)
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "x86_64-unknown-linux-gnu" ];
        };

        # Tauri dependencies
        tauriDeps = with pkgs; [
          webkitgtk_4_1
          gtk3
          cairo
          pango
          gdk-pixbuf
          glib
          dbus
          openssl
          librsvg
          libsoup_3
          at-spi2-atk
          atkmm
        ];

        tauriLibs = with pkgs; [
          webkitgtk_4_1
          gtk3
          cairo
          pango
          gdk-pixbuf
          glib
          dbus
          openssl
          librsvg
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain (for alakazam, calyrex, arceus)
            rustToolchain
            cargo-watch

            # Node.js (for giratina, arceus)
            nodejs_22

            # PostgreSQL (for alakazam)
            postgresql_16
            sqlx-cli
            pgadmin4-desktopmode

            # Google Cloud SDK (for alakazam GCS)
            google-cloud-sdk

            # Build dependencies
            pkg-config
            openssl
          ] ++ tauriDeps;

          shellHook = ''
            echo "Launched Arceus Dev Environment"

            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath tauriLibs}:$LD_LIBRARY_PATH"
            export PKG_CONFIG_PATH="${pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" tauriDeps}:$PKG_CONFIG_PATH"
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
          '';
        };
      }
    );
}
