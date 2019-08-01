# `create-smithy-app`

> A starter pack for writing and running Smithy apps!
>
> Smithy is a Rust framework for writing WebAssembly apps.
>
> [Learn more about Smithy](https://www.smithy.rs) or [view the source code](https://github.com/rbalicki2/smithy).

## About

This template is designed for creating Smithy apps.

It is a fork of the excellent [rust-webpack-template](https://github.com/rustwasm/rust-webpack-template).

## 🚴 Using This Template

You can use `npm init` to clone this template:

```sh
npm init smithy-app my-app
```

## 🔋 Batteries Included

This template comes pre-configured with all the boilerplate for compiling Rust
to WebAssembly and hooking into a Webpack build pipeline.

* `npm start` -- Serve the project locally for development at
  `http://localhost:8080`. It auto-reloads when you make any changes.

* `npm run build:prod` -- Export the project into the `/pkg` folder. These files
  can now be served statically, e.g. from a CDN.

* `npm run serve:prod` -- After `npm run build:prod` has been run, serves the
  files statically (using python. This may change soon to decrease the number
  of dependencies).

* `S3_BUCKET=... npm run upload` -- After `npm run build:prod` has been run,
  uploads the files to an S3 bucket with the correct mime types and with
  brotli compression.

* `npm test` -- Run the project's unit tests.
