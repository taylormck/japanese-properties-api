## Japanese Properties API

This is a simple API for uploading and retrieving data about Japanese real estate.

This server is a toy, and not meant for production use.
It uses a simple hash map to store data, and that data won't persist if the server restarts.

## Current Deployment

This app is currently deployed for cheap on Heroku.

It can be reached at [this URL](http://japanese-properties-api-4cc92a0116ec.herokuapp.com).

## API

```
# A simple up check to ensure the server is running
/up

# List all properties currently stored
/properties

# Show details for a specific property
/properties/:id

# Upload a CSV file with property data
# The file must be attached as as multipart/form-data
# For example:
#   curl ".../properties/upload" -F file=@sample.csv
#
# This will delete any existing data
/properties/upload
```

## Build from source

To build this project from source, you'll need to have Rust/Cargo installed.

```sh
git clone https://github.com/taylormck/japanese-properties-api
cargo build --release
```

## Running for local development

You can always run this project locally with cargo:

```sh
cargo run
```

There's also a `justfile` included, which has some additional handy tasks to run:

```sh
just server          # Run the server

just upload-sample   # Upload sample CSV data to the server

just download-sample # Download the sample data in JSON format

just view-sample     # Views the downloaded sample data using jq

just clean           # Delete the downloaded sample data
```

## Unimplemented

This project was built in a short amount of time as an exercise,
so there are a few features that aren't implemented:

- admin endpoints to manually edit or delete data
- user authentication
- shared data across instances
- persistent data storage
- reduce data size for structs in data store
- more thorough CSV file validation and error handling
- tracing and logging
- testing
