# Nix for Quasar

Nix for Quasar is a framework for building and managing blockchain applications, supporting several chains such as:

- quasar (v0.1.1)
- gaia (v9.1.1)
- osmosis (v15.0.0)
- quicksilver (v1.2.13)

You can easily inspect the content of the flake with the following commands:

```bash
nix flake show
nix flake check
```

## Getting started

- Installing nix
- Enabling experimental features

## Building with Nix Shell

Here's how to build your preferred application, for instance, `quasar`:

Navigate into the 'nix' directory:

```bash
cd nix
```

Execute the build command for your chosen app:

```bash
nix build .#quasar
```

Upon successful completion, the above steps will create a `result` symlink inside the `nix` folder. You can then access
the built binary by executing:

```bash
./result/bin/quasarnodedd
```

---

## Building with Docker

For building all the apps at once with Docker, follow these steps:

Navigate into the 'nix-docker' directory:

```bash
cd nix-docker
```

Initiate the build process:

```bash
nix build .#quasarImage
```

Load the built images located inside the `result` directory:

```bash
docker load -i ./result
```

To verify that the images are correctly loaded into Docker, list your Docker images with the following command:

```bash
docker images
```

Lastly, you can run each image with the following command. Replace `quasar:latest` with your desired image tag:

```bash
docker run -it --rm quasar:latest
```