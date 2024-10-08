{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, ... }:
    # TODO: cross compilation
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook4
          ];

          buildInputs = with pkgs; [
            webkitgtk_6_0
            libsoup
            openssl
            glibc
            glib-networking
          ];
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        webkit-pdf-inator = craneLib.buildPackage (commonArgs // {
          meta.mainProgram = "webkit-pdf-inator";
          inherit cargoArtifacts;
        });

        dockerImage = pkgs.dockerTools.buildImage {
          name = "webkit-pdf-inator";
          copyToRoot = pkgs.buildEnv {
            name = "webkit-pdf-inator-env";
            paths = [ webkit-pdf-inator ];
            pathsToLink = [ "/bin" ];
          };
          config = {
            Cmd = [ "${lib.getExe webkit-pdf-inator}" ];
          };
        };
      in
      {
        checks = {
          inherit webkit-pdf-inator;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          cargo-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          cargo-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          cargo-fmt = craneLib.cargoFmt {
            inherit src;
          };

          cargo-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `webkit-pdf-inator` if you do not want
          # the tests to run twice
          nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          default = webkit-pdf-inator;
          dockerImage = dockerImage;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = webkit-pdf-inator;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          GIO_EXTRA_MODULES = "${pkgs.glib-networking}/lib/gio/modules";
        };
      });
}
