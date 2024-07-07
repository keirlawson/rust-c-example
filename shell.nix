{nixpkgs ? import <nixpkgs> {}}:
with nixpkgs;
  mkShell {
    buildInputs = [gcc libclang clang-tools];
    LIBCLANG_PATH = "${libclang.lib}/lib";
    shellhook = ''
    '';
  }
