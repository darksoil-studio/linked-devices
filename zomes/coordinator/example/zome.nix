{ inputs, ... }:

{
  perSystem = { inputs', system, self', ... }: {
    packages.example = inputs.tnesh-stack.outputs.builders.${system}.rustZome {
      workspacePath = inputs.self.outPath;
      crateCargoToml = ./Cargo.toml;
      cargoArtifacts = inputs'.tnesh-stack.packages.zomeCargoArtifacts;
    };

  };
}

