with import ./nix/pkgs.nix {};

stdenv.mkDerivation rec {
  name = "web3-hd";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
  ];
}
