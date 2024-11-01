{ inputs, ... }:

{
  perSystem = { inputs', system, self', ... }: {
    packages.linked_devices =
      inputs.hc-infra.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        cargoArtifacts = inputs'.hc-infra.packages.zomeCargoArtifacts;
      };

  };
}

