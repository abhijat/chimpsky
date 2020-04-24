
chimpsky is a program to parse a set of files conforming to [JSON-SCHEMA](https://json-schema.org/understanding-json-schema/index.html), and optionally produce random payloads from the parsed schema.


Point it to the root of your directory containing schema files, which may or may not refer to each other.


chimpsky can either report the objects found in the schema path, or pick one and produce random payloads for it.

```
~/dev/rust/chimpsky  (master) 
 abhijat $ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/chimpsky --help`
chimpsky 0.1.0

USAGE:
    chimpsky [FLAGS] --schema-dir <schema-dir> <SUBCOMMAND>

FLAGS:
    -h, --help               Prints help information
    -r, --report-and-exit    Exit after showing analyzed object definitions
    -V, --version            Prints version information

OPTIONS:
    -s, --schema-dir <schema-dir>    Root path for json schema files

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    randomize    Generate random JSON payloads based on supplied object name
    report       Print object definitions found and exit
```


#### Example runs:

##### Reporting the examined objects in schema path

```
~/dev/rust/chimpsky  (master) 
 abhijat $ cargo run -- -s schema report
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/chimpsky -s schema report`
a_carried_object in file a_carried_object.schema.json#/definitions/a_carried_object
bigmessage.schema in file bigmessage.schema.json#/definitions/bigmessage.schema
root_message_format in file root_message_format.schema.json#/definitions/root_message_format

```

##### Generating a random payload for one object
```
~/dev/rust/chimpsky  (master) 
 abhijat $ cargo run -- -s schema randomize -o bigmessage.schema.json#/definitions/bigmessage.schema -p -e1 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/chimpsky -s schema randomize -o 'bigmessage.schema.json#/definitions/bigmessage.schema' -p -e1`
{
  "a_carried_object": {
    "some_date_of": "2254-04-06T00:48:00+00:00",
    "version": 914.8072311384599
  },
  "action": "puZubrkxFlFnIhDNuY7p",
  "category_of": "yjZAxsCJvCznbgkUNHe8",
  "data": {
    "0xvhRPxzpazoa7Gj1a3C": "hwcsP3BBQPtR2iEcpa9n",
    "4Zg0wBloPnq4V6GyKaqe": true,
    "5w61a1oZttqAZDrFY2NC": "vGDm7mvyLhwcMDUiCNsP",
    "IMfQDkOXq5zYWK0Mh9Yz": false,
    "cBLcVoENuYKdDDqwDKFo": "HVN8l72qWUijzu3SsltH",
    "djxXvvarNHnGxfOrylMx": "ZGYFryuJc5zziFuQEMFJ",
    "fUQax6ZIMj26p4yN3Sal": "97aK6fsIKXVG64Y4d7Q4",
    "lzjM2UhsKmbn85qMc2LC": 8759,
    "n9LPgHShClZDBcK4e1TB": true,
    "zRIGKXe0MYXeHeNnjL6S": 5066
  },
  "room_number": "w-9z5ibp-8zls3",
  "the_name": "nYmntc8ZEPa4sGGH5uO6",
  "the_payload": {
    "1UjT3Y70Lf0VcdgHpnfe": false,
    "DFH2njvfrDkVxQBizpBF": false,
    "EEU7ynyzhs2batYq52II": "s1oXNyD83RFxS7QxNrD9",
    "NkxLCE1AxMGWV2rGBbd3": 1841,
    "NnaroRsCXXmEq01cotIl": "wWMKTJ35Kg7AVlvlhlVK",
    "ZW2NNE0ayc9vdLQhp882": true,
    "gfJQ0v6DDpruU9PvIl6G": "SUfprYx08hw2KVN7ossO",
    "mosfM4aZQglT5vcCTzLg": "uovThPaTIC1yPprKFI73",
    "wIyMrI7AY8jPglUfuN2e": 4019,
    "x9uZhzxmWG9tzd93YjAq": "BU0FPEmTt0DGpIpNbYUG"
  },
  "timestamp": "2254-05-20T07:12:00+00:00",
  "type": "wlSLGf4opUMZO8goCUqt",
  "unique_number": 2362
}
```

Increase the emission count to get more payloads. The `schema` directory contains a couple of sample schemas.
