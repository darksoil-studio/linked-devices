{ inputs, ... }:

{
  perSystem = { inputs', system, ... }: {
    packages.linked_devices_integrity =
      inputs.tnesh-stack.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        cargoArtifacts = inputs'.tnesh-stack.packages.zomeCargoArtifacts;
      };
  };
}

