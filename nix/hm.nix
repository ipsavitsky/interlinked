{ interlinked }:
{
  pkgs,
  config,
  lib,
  ...
}:
let
  cfg = config.programs.interlinked;
in
{
  options.programs.interlinked = {
    enable = lib.mkEnableOption "interlinked";

    package = lib.mkOption {
      type = lib.types.package;
      default = interlinked.${pkgs.stdenv.hostPlatform.system}.cli;
      description = "The interlinked package to use";
    };

    settings = lib.mkOption {
      type = lib.types.attrs;
      default = { };
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."interlinked/config.toml" = lib.mkIf (cfg.settings != { }) {
      source = (pkgs.formats.toml { }).generate "config.toml" cfg.settings;
    };
  };
}
