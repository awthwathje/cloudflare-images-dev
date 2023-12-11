# Cloudflare Images Dev
A project to mock the [Cloudflare Images](https://www.cloudflare.com/developer-platform/cloudflare-images/) API, for local development purposes.

## Purpose & limitations

The idea of this project is to mimic the API of Cloudflare Images locally. Not the full functionality of the original service is mocked. This service does resize the images to specified dimensions, but don't do any further optimizations, such as format negotiation and metadata stripping, since they are not needed during the development anyway. The goal is to allow to develop locally, without the need to rely on a cloud.

Some errors are also mocked, for example when an unknown variant is requested.

Although this service is supposed to be ran in Docker, it is also possible to run it as a binary built using a native Rust compiler in any supported OS.

## Run using a pre-built image from GitHub Packages (recommended)

This project has an Actions pipeline set up to automatically build a Docker image every time the project is updated. This way you don't have to build the service yourself and can start using it immediately.

Example `docker-compose.yml`:

```yaml
version: '3.8'
services:
  cloudflare_images_dev:
    image: ghcr.io/awthwathje/cloudflare-images-dev
    environment:
      - HOSTNAME=127.0.0.1
      - PORT=3030
      - ACCOUNT_ID=account-id
      - ACCOUNT_HASH=account-hash
      - VARIANTS=variant1_1920_1080,variant2_640_480,variant3_40_40
    ports:
      - 3030:3030
    volumes:
      - ./local-seed-dir:/.files # optional mount with pre-seeded images
```

Run `docker-compose up` to start the container.

## Build it yourself

If preferred, this repo can be cloned and the project can be built manually. There are two options.

### Using locally-available Rust compiler

If you already have Rust compiler installed, and assuming `cargo` command is available locally:

- Use `cargo run` to build & run the project in one go.

The default variables from `constants.rs` will be used, unless you pass them as env variables, for example like this `ACCOUNT_ID="mocked-account-id" VARIANTS="foo,bar,baz" cargo run`.

### Using Docker

You can build and run your own Docker image, so you don't have to manage the dependencies yourself (such as Rust).

- Build the container: `docker build --tag cloudflare_images_dev .`.
- Run the container: `docker run --rm --env HOSTNAME=127.0.0.1 --env PORT=3030 --publish 3030:3030 cloudflare_images_dev`.

Or, if you prefer Docker Compose:

Example `docker-compose.yml`:

```yaml
version: '3.8'
services:
  cloudflare_images_dev:
    build: .
    environment:
      - HOSTNAME=127.0.0.1
      - PORT=3030
      - ACCOUNT_ID=account-id
      - ACCOUNT_HASH=account-hash
      - VARIANTS=variant1_1920_1080,variant2_640_480,variant3_40_40
    ports:
      - 3030:3030
    volumes:
      - ./local-seed-dir:/.files # optional mount with pre-seeded images
```

Run `docker-compose up` to start the container.

# Environment variables

| Variable     | Default Value | Description |
|--------------|---------------|-------------|
| HOSTNAME     | 127.0.0.1     | The hostname where the service is running. |
| PORT         | 3030          | The port on which the service is listening. |
| ACCOUNT_ID   | account_id    | The account ID for the Cloudflare Images service. |
| ACCOUNT_HASH | account_hash  | The account hash for the Cloudflare Images service. |
| VARIANTS     | variant1_1920_1080,variant2_640_480,variant3_40_40 | A comma-separated list of image variants. Format: `variantName_width_height`. |
