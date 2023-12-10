# eylab-csv

A cli tool to parse large JSON files to CSV. Developed by Per Henrik for Eylab Corp.
A cli tool to remove columns from a CSV file.

## How to use

Just download the project and build it for your architecture, then you can run it from the terminal.

```bash
./eylab-csv path/to/file_to_edit.csv path/to/file_to_create.csv --skip 2-4,8,10-43
```

### Skip

The format for skipping columns is pretty simple, each group of skips are separated with a comma(","). And each group contains one number or a range between two numbers signified with a dash ("-"). The skip is inclusive as well.

So the string `2-4,6,8-10` will skip columns 2, 3, 4, 6, 8, 9 and 10 and keep columns, 1, 5, 7 and whatever comes after 10.
