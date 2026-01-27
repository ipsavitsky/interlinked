{ interlinked }:
{
  pkgs,
  lib,
  config,
  ...
}:
let
  cfg = config.services.interlinked;
in
{
  options.services.interlinked = {
    enable = lib.mkEnableOption "A link shortener with PoW verification";

    package = lib.mkOption {
      type = lib.types.package;
      inherit (interlinked.${pkgs.stdenv.hostPlatform.system}) default;
    };

    address = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1:8080";
    };

    url = lib.mkOption {
      type = lib.types.str;
      default = "http://localhost:5000";
    };

    dataDir = lib.mkOption {
      type = lib.types.path;
      default = "/var/lib/interlinked";
    };

    logLevel = lib.mkOption {
      type = lib.types.str;
      default = "INFO";
    };

    difficulty = lib.mkOption {
      type = lib.types.int;
      default = 2;
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "interlinked";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "interlinked";
    };
  };

  config = lib.mkIf cfg.enable {

    users.users = lib.mkIf (cfg.user == "interlinked") {
      interlinked = {
        inherit (cfg) group;
        isSystemUser = true;
      };
    };

    users.groups = lib.mkIf (cfg.group == "interlinked") {
      interlinked = { };
    };

    systemd.tmpfiles.rules = [
      "d '${cfg.dataDir}' 0700 ${cfg.user} ${cfg.group} - -"
    ];

    systemd.services.interlinked = {
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/itlkd-server";
        Environment = [
          "INTERLINKED_ADDRESS=${cfg.address}"
          "INTERLINKED_URL=${cfg.url}"
          "INTERLINKED_DB_URL=${cfg.dataDir}/interlinked.sqlite"
          "INTERLINKED_LOG_LEVEL=${cfg.logLevel}"
          "INTERLINKED_DIFFICULTY=${builtins.toString cfg.difficulty}"
        ];
        Restart = "on-failure";
        User = cfg.user;
        Group = cfg.group;
      };

      wantedBy = [ "multi-user.target" ];
    };
  };
}
