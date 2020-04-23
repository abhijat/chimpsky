
chimpsky is a program to parse a set of files conforming to [JSON-SCHEMA](https://json-schema.org/understanding-json-schema/index.html), and optionally produce random payloads from the parsed schema.


Point it to the root of your directory containing schema files, which may or may not refer to each other.


Example runs:

##### Reporting

```
~/dev/rust/chimpsky  (master) 
 abhijat $ ./target/release/chimpsky -s schema report
a_carried_object in file metadata.schema.json#/definitions/a_carried_object
root_message_format in file root_message_format.schema.json#/definitions/root_message_format
bigmessage.schema in file bigmessage.schema.json#/definitions/bigmessage.schema
```

##### Generating random payload
```
~/dev/rust/chimpsky  (master) 
 abhijat $ ./target/release/chimpsky -s schema randomize -e 1 -o bigmessage.schema.json#/definitions/bigmessage.schema -p
{
  "action": "nDra2PUoATEZbV4G431T",
  "category_of": "GEvH9uPZAfHiCNLucVtB",
  "data": {
    "5pQXetwvMPIMTeNhC7ya": "f3L5H76E8L0sEe9BwDwE",
    "9sz7sGlUJZBp6NUndrTk": 3024,
    "KyLYLERdIpUuy57mFMqg": 5994,
    "VEEnu7mz2mD12Fw0Ibds": "20rO1FDwyr1GlAOiLRgQ",
    "ZQ6Wd1Ylb0IXeinWYuCU": false,
    "Zn802O021VqhjLXFYLGw": false,
    "gfelhDIGs4wcevaMxcnH": 1422,
    "h9KVHbt8GfTKLl6jGBuD": true,
    "mTZJZKwKf3cwD6dYS90R": true,
    "v0ONHg822L6AU0Zqcv1h": true
  },
  "room_number": "rhzibv-6praqk-8lxfg-t",
  "the_name": "O3bksbZ8oNiHD0YJn2Xg",
  "the_payload": {
    "9ECYBjkAFUxHr72c3lTY": 468,
    "Mjr3XXTeG68EE9bjgtqX": false,
    "MxF6BfKZMJ81GHGu6ooX": true,
    "Zt6m6ZV4PQkANXVKPi13": false,
    "a8TdpGDSkeYoRvGElPCp": false,
    "f8N7ZxKrOQDlTdBmA7Ah": "9nz0eVM01aMJi1pDAO4z",
    "r7MoQVRY5TCVon3uatBr": 9151,
    "suUfoEzhI08uEzK1vvEC": true,
    "tVm70aDExkcXXIyUojYY": true,
    "wZlmjCOsoy9cVhuTwAT3": false
  },
  "unique_number": 7375
}
```

Increase the emission count to get more payloads. The `schema` directory contains a couple of sample schemas.
