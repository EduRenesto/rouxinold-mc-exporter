{ lib
, pkgs
, config
, ...
}: with lib; {
  options.services.rouxinold-mc-exporter = {
    enable = mkEnableOption "rouxinold-mc-exporter";
    envFile = mkOption {
      type = types.str;
      default = "/opt/rouxinold/.env-mc-exporter";
    };
  };

  config = let
    rouxinold-mc-exporter = config.services.rouxinold-mc-exporter;
  in {
    systemd.services.rouxinold-mc-exporter = mkIf rouxinold-mc-exporter.enable {
      wants = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      enable = true;
      environment = {
        "ROUXINOLD_ENV_FILE" = rouxinold-mc-exporter.envFile;
      };
      serviceConfig = {
        ExecStart = "${pkgs.rouxinold-mc-exporter}/bin/rouxinold-mc-exporter";
        Restart = "always";
        User = "rouxinold-mc-exporter";
        Group = "rouxinold-mc-exporter";
      };
    };

    users.users.rouxinold-mc-exporter = {
      name = "rouxinold-mc-exporter";
      group = "rouxinold-mc-exporter";
      isNormalUser = true;
    };
    users.groups.rouxinold-mc-exporter = {};
  };
}
