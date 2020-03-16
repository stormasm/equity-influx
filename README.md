
read each file in
[equity-history](https://github.com/stormasm/equity-history)

and create a file similar to this one...   

here is an example of a influxdb file...   
influx-examples/data/temp{1,2}.txt

here is what the influx filename should look like after it gets created.

```
symbol-name volume=31,close=99 1561930347
```

##### things to do   

[Use this example to read the csv file](https://docs.rs/csv/1.1.3/csv/struct.Reader.html#example)

* convert the timestamp in the csv file to the influxdb time format

[ex02.rs](https://github.com/stormasm/rust-examples/blob/master/fileio/examples/ex02.rs) contains a dir reader which reads all of the files in a directory

##### references

[line protocol details](https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/)
