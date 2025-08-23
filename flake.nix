# Based on swww's flake: https://github.com/LGFae/swww
{
    description = "A Rust remake of Gang Garrison 2";
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };
    outputs = {
        self,
        nixpkgs,
        rust-overlay,
        ...
    }: let
        inherit (nixpkgs) lib;
        systems = [
            "x86_64-linux"
            "aarch64-linux"
            "x86_64-darwin"
            "aarch64-darwin"
        ];
        pkgsFor = lib.genAttrs systems (system:
            import nixpkgs {
                localSystem.system = system;
                overlays = [(import rust-overlay)];
            });
        cargoToml = lib.importTOML ./Cargo.toml;
    in {
        packages = lib.mapAttrs (system: pkgs: {
            gang-garrison-rs = let
                rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
                rustPlatform = pkgs.makeRustPlatform {
                    cargo = rust;
                    rustc = rust;
                };
            in
                rustPlatform.buildRustPackage.override {
                    stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
                } rec {
                    pname = "gg2-custom-client";
                    src = pkgs.nix-gitignore.gitignoreSource [] ./.;
                    inherit (cargoToml.workspace.package) version;
                    cargoLock = {
                        lockFile = ./Cargo.lock;
                        outputHashes = {
                            # TODO: Publish string path
                            "string-path-0.1.0" = "sha256-wACM/gJNqn5dNb2e4b7iydLu/0PY+JMDmygIjerJo6c=";
                        };
                    };
                    buildInputs = with pkgs; [
                        udev
                    ];
                    doCheck = false;
                    nativeBuildInputs = with pkgs; [
                        pkg-config
                    ];
                    meta = {
                        description = "A Rust remake of Gang Garrison 2";
                        license = lib.licenses.gpl3;
                        platforms = lib.platforms.linux;
                        mainProgram = "gg2-custom-client";
                    };
                };
            default = self.packages.${system}.gang-garrison-rs;
        })
        pkgsFor;
        formatter = lib.mapAttrs (_: pkgs: pkgs.alejandra) pkgsFor;
        devShells = lib.mapAttrs (system: pkgs: {
            default = pkgs.mkShell.override {
                stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
            } {
                inputsFrom = [self.packages.${system}.gang-garrison-rs];
            };
        })
        pkgsFor;
        overlays.default = final: prev: {inherit (self.packages.${prev.system}) gang-garrison-rs;};
    };
}
