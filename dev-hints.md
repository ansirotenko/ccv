
# Update brew

1. after successfull release find sha for amd and intel architectures for macOS generateed at ./.github/workflows/release.yml 
2. make a commit to https://github.com/ansirotenko/homebrew-repo with updated sha's and version

# Update snap

1. Do not forget to update version snapcraft.yaml
2. Builds for snap are done automatically after each commit to master.
3. find proper build at snapcraft https://snapcraft.io/ccv-clip-manager/builds
4. promote new release to stable here https://snapcraft.io/ccv-clip-manager/releases 

# Generate icons

1. replace `src-tauri/ccv/icons/original.png`
2. cd to `src-tauri/ccv`
3. run `cargo tauri icon ./icons/original.png`
4. replace `public/favicon.ico`
5. replace `src/assets/logo256.png`

# Create sqlite db
- run `cargo install diesel_cli --no-default-features --features "sqlite-bundled"`
- cd to `src-tauri\ccv_sqlite`
- run `diesel setup --database-url=ccv.db`
- run `diesel migration run --database-url=ccv.db`

# Local run
- direct option `cargo tauri dev`
- alterative option
    - run `npm run dev`
    - cd to `src-tauri`
    - run `cargo run`
- debug
    - run `npm run dev`
    - press f5 at vscode

# Build at docker
Here is recipe how ccv can be build in docker. Unfortunately the only target OS is Linux.

- build docker image
    ```bash
    docker build . -t ccv_builder:local -f ./scripts/docker_build/Dockerfile
    ```
- run build
    ```bash
    ./scripts/docker_build/run.sh x86_64-unknown-linux-gnu
    ```
- remove image in case you dont need it anymore
    ```bash
    docker rmi ccv_builder:local
    ```
# Profiling budle
To see the final bundle size
- install carg-bloat [if it is first time]
```bash
cargo install cargo-bloat
```
- execute comand 
```bash
cd ./src-tauri
cargo bloat --release --crates
```

To see the final ui-bundle graph
- build ui
```bash
npm run build
```
- then open `./profiler/ui-bundle-stats.html` to see ui-bundle bundle graph

