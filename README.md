# Infinite Monkey

A tool for generating random JSON data with a specified structure.


## CLI arguments
* -c / --config: Config file location.
* -o / --output: Output file location.
* -d / --debug: Enable debut output.


## Configuration

See the `example_config.toml` file for an overview of the various types of columns and pools that can be used.

### Columns
This collection defines the columns that will be generated on each row.

### Pools
Pools are collections of data that are used with certain types of columns. The names of the pools are used in columns of the "comma-separated" type.

