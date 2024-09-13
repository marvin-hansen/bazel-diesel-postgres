# bazel-diesel-postgres

Bazel configuration to build a Rust crate that uses Diesel with custom array data types in Postgres.

A cargo-only version of this repo has recently (August 2024) been contributed to the Diesel project. 
See [PR #4169](https://github.com/diesel-rs/diesel/pull/4169) for more details. 

### Setup:

1) Start a Postgres server with database postgres and password postgres, either locally or in Docker i.e.

`docker run --name postgres-5432 -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres:16.3-bookworm`

Any postgres version past version 14 should suffice. Versions before 14 may work but have not been tested.

2) Add an .evn file to the repo root with the DB URL:

`echo "POSTGRES_DATABASE_URL=postgres://postgres:postgres@localhost/postgres" > .env`

Note, if your postgres server uses a different default database or password,
ensure you update the POSTGRES_DATABASE_URL in the .env file accordingly.

To build the crate with Bazel, ensure you have [Bazelisk installed](https://github.com/bazelbuild/bazelisk/releases).

3) Run the example

Note, the tests run an embedded database migration to create all tables, types, and schema meaning
this example is self-contained given a working Postgres server is accessible.

a) Cargo:

Build the project:

`cargo b`

Run all tests:

`cargo t`

b) Bazel:

`bazel build //...`

Run all tests:

`bazel test //...`


If you want to preserve CARGO compatibility i.e. for better IDE support in JetBrains RustRover, 
ensure your Bazel dependencies match exactly those in the Cargo.toml file.

### Embedded DB Migrations

Conventionally, you would use the `diesel_migrations` macro to manage your embedded database migrations.
However, because of
a very [unfortunate discrepancy related to how Bazel via PWD and Cargo set the CARGO_MANIFEST_DIR](https://internals.rust-lang.org/t/build-scripts-pwd-and-cargo-manifest-dir-strangeness/16833/2)
environment variable required by the macro, the `diesel_migrations` crate does not work in Bazel. 
There is no good solution available for the time being. A pull request with a solution is very much welcome.

The main implication of this issue is that the macro cannot bundle all migrations automatically into an migration embedding, 
but still can perform the migrations from a given migration embedding.
Therefore you have to write a custom migration embedding as shown in the [embed_migration file](src/embed_migrations.rs)
to ensure embedded migrations are applied automatically before tests and when your application starts.
The CARGO_MANIFEST_DIR environment variable used in the custom migration is overwritten by Bazel to ensure 
that the project compiles with both, Cargo and Bazel. 

In a larger multi-crate project you want to rename that environment variable in the Bazel BUILD file 
to something more obvious i.e. DIESEL_MIGRATION_DIR, but doing so requires you to ensure Cargo 
also sets an environment variable of the same name if you want to preserve that
Cargo and Bazel build the same project. 

In a Bazel only project, however, you can simplify the environment variable by replacing 
the genrule with a location lookup i.e.:

```python
copy_directory(
     name = "copy_migrations",
     src = "migrations",
     out = "migrations",
 )
 
rust_library(
    name = "pg_smdb",
    srcs = glob([
        "src/**",
    ]),
    compile_data = [
        ":copy_migrations",
    ],
    rustc_env = {
        "DIESEL_MIGRATION_DIR": "$(location :copy_migrations)",
    },
    crate_root = "src/lib.rs",
    visibility = ["//visibility:public"],
    deps = [ ..
    ],
) 
```
Note, in that case, all files and folders are relative to the migrations folder so you would have to access them as shown below:

```rust 
const DIESEL_UP: &'static str = include_str!(concat!(
env!("DIESEL_MIGRATION_DIR") // Path relative to the migrations folder
"/00000000000000_diesel_initial_setup/up.sql"
));
```

If you work in a Cargo only project and don't need Bazel, you can uncomment the `diesel_migrations` macro  in lib.rs and delete the custom embed_migrations file.
