{
  description = "mm-schema";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;

      toolchain =
        pkgs.rust-bin.stable.latest.default.override
        {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
        };

      craneLib = crane.mkLib pkgs;

      sqlFilter = path: _type: builtins.match ".*sql$" path != null;
      compositeFilter = path: type: (sqlFilter path type) || (craneLib.filterCargoSources path type);

      src = pkgs.lib.cleanSourceWith {
        src = craneLib.path ./.; # The original, unfiltered source
        filter = compositeFilter;
      };

      # Common arguments can be set here to avoid repeating them later
      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs = with pkgs;
          [
            pkg-config
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
          ];

        # Additional environment variables can be set directly
        # MY_CUSTOM_VAR = "some value";
      };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      mm-schema = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });
    in {
      checks = {
        # Build the crate as part of `nix flake check` for convenience
        inherit mm-schema;

        # Run clippy (and deny all warnings) on the crate source,
        # again, reusing the dependency artifacts from above.
        #
        # Note that this is done as a separate derivation so that
        # we can block the CI if there are issues here, but not
        # prevent downstream consumers from building our crate by itself.
        mm-schema-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        mm-schema-doc = craneLib.cargoDoc (commonArgs
          // {
            inherit cargoArtifacts;
          });

        # Check formatting
        mm-schema-fmt = craneLib.cargoFmt {
          inherit src;
        };

        # Audit dependencies
        mm-schema-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };

        # Audit licenses
        mm-schema-deny = craneLib.cargoDeny {
          inherit src;
        };

        # Run tests with cargo-nextest
        # Consider setting `doCheck = false` on `mm-schema` if you do not want
        # the tests to run twice
        mm-schema-nextest = craneLib.cargoNextest (commonArgs
          // {
            inherit cargoArtifacts;
            # partitions = 1;
            # partitionType = "count";
          });
      };

      packages = {
        default = mm-schema;
      };

      apps.default = flake-utils.lib.mkApp {
        drv = mm-schema;
      };

      devShells.default = craneLib.devShell {
        # Inherit inputs from checks.
        checks = self.checks.${system};

        # Additional dev-shell environment variables can be set directly
        # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

        shellHook = ''
          # For rust-analyzer 'hover' tooltips to work.
          export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library";
        '';

        packages = with pkgs; [
          just
          zellij # tmux alternative
          sqlite
          sqlitebrowser
          aider-chat
        ];
      };
    });
}
