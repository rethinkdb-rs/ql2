with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "ql2";
  buildInputs = [ gcc protobufc ];
}
