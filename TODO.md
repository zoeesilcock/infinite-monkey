# Todo


## Split out each column type to their own methods
* The aim is to allow composing different sets of columns instead of the fixed example we have now in `generate_row`.

## Allow using a config file to define the structure of the fake data
* https://toml.io/en/
* Allow defining data pools to be used in columns
* Allow defining a list of columns
    * Name
    * Data type
    * Data length
* Reference
    * https://docs.rs/toml/latest/toml/
    * https://serde.rs/enum-representations.html
    * https://stackoverflow.com/questions/69767906/serde-deserialize-a-field-based-on-the-value-of-another-field

## Implement error handling


# Done

## Separate the data generation logic into lib files
* Call the lib from the main.rs file

## Use a full ISO 8601 date time in UTC
* https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON

## Convert to JSON
* Add serde_json
* Add a top level key
* Write result to a file

## Don't include the current row id in references.

## Don't recreate pools for each row
* The idea is that they re-use the same pools.

## Basic types of data
* Sequencial number
* Date
* Random text
* Comma separated text
* Hierarcical data (A, A1, A2, B, B1, B2)
* Reference to other row

