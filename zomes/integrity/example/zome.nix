{ inputs, ... }:

{
  perSystem = { inputs', system, ... }: {
    packages.example_integrity =
      inputs.hc-infra.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        cargoArtifacts = inputs'.hc-infra.packages.zomeCargoArtifacts;
      };
  };
}

