```bash
nix build
```

```bash
docker load -i ./result
```

```bash
docker images
```

Only for Mac Silicon users:

1. Enable on Docker Desktop > Settings > Features in development > Use Rosetta for x86/amd64
emulation on Apple Silicon which Turns on Rosetta to accelerate x86/amd64 binary emulation on Apple Silicon. Note - you
must have the Virtualization Framework enabled (via the toggle on the General panel).
2. ... continue

```bash
docker run -it --rm quasar:latest
```