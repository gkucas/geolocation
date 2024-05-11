# IP Geolocator

## Information

* Programming language - Rust(tested 1.77.2)
* Development OS - macOS Sonoma 14.4.1

## Design
The design is oriented into lookup speed over memory or preprocessing time. Lookup time could be said to be constant.
Lookup is based on the facts that ranges can be treated as sequential and the smallest covered range is of size 256.
This allows to list all such ranges. If one location covers a multiple of 256 than this location is indexed
multiple times in the DB depending on formula: 

(max - min) / 256

Given ranges:
* 0..255
* 256..511
Then DB contents:
```
0..255
256..511
256..511
```

First prototype was implemented using arrays and optimized using string pool. However, memory consumption was 72mb.
Final version performs lookup directly on disk using seek. Having location index in file it is required that each location
would take the same amount of space, provided database file takes 46 bytes, so smaller locations are padded to reach this size.

## Building and running

* Open a terminal in the project root
* Run ``cargo build --all --release``
* Run ``./target/debug/transformer database.csv`` (database should be located in the current directory or provide relative/absolute path) 
* Run ``python geolocation_test.py --database database.csv --executable target/release/geolocator``

## Result

Testing result vary from 20 to 60 points on Apple M1 Pro:
```shell
Database loaded Memory usage: 1.25mb Load time: 17μs
OK    1.0.0.0 US Los Angeles Memory usage: 1.25mb Lookup time: 12μs
OK    71.6.28.0 US San Jose Memory usage: 1.25mb Lookup time: 9μs
OK    71.6.28.255 US San Jose Memory usage: 1.25mb Lookup time: 8μs
OK    71.6.29.0 US Concord Memory usage: 1.25mb Lookup time: 7μs
OK    53.103.144.0 DE Stuttgart Memory usage: 1.25mb Lookup time: 7μs
OK    53.255.255.255 DE Stuttgart Memory usage: 1.25mb Lookup time: 10μs
OK    54.0.0.0 US Rahway Memory usage: 1.25mb Lookup time: 8μs
OK    223.255.255.255 AU Brisbane Memory usage: 1.25mb Lookup time: 8μs
OK    5.44.16.0 GB Hastings Memory usage: 1.25mb Lookup time: 7μs
OK    8.24.99.0 US Hastings Memory usage: 1.25mb Lookup time: 7μs
Final points for 10 measurements:  20.817
```
## Warning

Sometimes the program might exit faster than test script is able to capture memory. For more consistent
result you can uncomment geolocator/src/main.rs:52 sleep and use this version of the program.