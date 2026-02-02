_: {
  projectRootFile = "flake.nix";
  programs = {
    nixfmt.enable = true;
    statix.enable = true;
    deadnix.enable = true;
    rustfmt.enable = true;
    sqlfluff = {
      enable = true;
      dialect = "sqlite";
    };
    prettier.enable = true;
    zizmor.enable = true;
  };
}
