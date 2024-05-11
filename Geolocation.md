# A challange for Great Low Level Developer

## Prerequisites
You should have received these files:
- Geolocation.md
- geolocation/database.csv
- geolocation/sample_app.cpp
- geolocation/geolocation_test.py
- geolocation/CMakeLists.txt

## Specification
Using systems programming language of your choice (C/C++/rust/go/zig...) build
a command line application that performs geolocation lookups for given IP addresses.

To do the lookup, use the supplied geolocation information from .csv file, (later
on referred to as "database" file).

### The Protocol
Your application is expected to implement specific line-based request-response
protocol for communication with other processes.

The protocol itself consists only of three commands:
* LOAD
* LOOKUP <IPv4 address>
* EXIT

#### LOAD
LOAD command is used for loading the database into memory. The application
should respond with the string `OK` once the database is fully loaded. Note, as
this is line-based protocol, the string `OK` must end with new line.

Example command execution:
```
> LOAD
< OK

```

Here `>` means data that your app receives via stdin, and `<` means data your
app writes into the stdout.

For error cases, "ERR" should be printed as a response, for example:
```
> LOAD
< ERR

```
No need to go deeply into the error codes/messages etc. just an "ERR" string is enough.

#### LOOKUP
LOOKUP command is used for performing geolocation lookup. The application
should respond with the location formated as `<COUNTRY CODE>,<CITY>`, there
should be no spaces before or after the comma. The response must end with
new line.

Example command execution:
```
> LOOKUP 71.8.28.3
< US,Hammond

```

Here `>` means data that your app receives via stdin, and `<` means data your
app writes into the stdout.

For error cases, "ERR" should be printed as response, for example:
```
> LOOKUP 1.2.3.4.5
< ERR
```
No need to go deeply into the error codes/messages etc. Just an "ERR" string is enough.


#### EXIT
EXIT command indicates that the application should now exit. Note that before
exiting, the application should respond with OK. The response must end with
new line.

Example command execution:
```
> EXIT
< OK

```

Here `>` means data that your app receives via stdin, and `<` means data your
app writes into the stdout.

---

You can use the provided `geolocation/geolocation_test.py` script to test the
compatability of your implementation.

### The Goal
Your main goal is to make this geolocation app as efficient as possible. Any
and all optimizations are on the table as long as CLI protocol is adhered and it
can help lower the resource usage.

More specifically by "resource usage" we mean
* load time
* lookup time
* memory usage
in these proportions:

```
points = load_time_ms + memory_usage_mb * 10 + lookup_time_ms * 1000
```

The goal is to collect as low amount of points as possible.

In order to achieve the best performance, you are encouraged to preprocess the
database file into your own, custom, on-disk format, which will be used during
database loading.

The same goes for in-memory data structures. You are encouraged to research and
implement any structures/algorithms which will help you lower the points.

While Your solution should handle other datasets gracefully, optimizations can and
should be tailored to this specific dataset. In case the optimization is not compatible
with other datasets one can just return an error during preprocessing or loading.

In general any optimization is allowed as long as original geolocation_test.py
script (or the CLI protocol) is not modified. Even ones which feels like cheating.

Therefore choosing a good approach for this task is the most important choice, so
do not be afraid to use your creativity and _destroy_ the numbers!

Note: measurements will be performed on hot page-cache.

### Requirements
- Provide README, explaining how to compile and setup the program
- Specify operating system it is designed for
- Document your approach at solving the problem, especially in-memory
  and on-disk data structures
- If you choose to use preprocessed database format, provide a script or some
  program (with source code) which was used to do preprocessing.
- Provide expected performance figures (in case we would get lower speeds
  due to some minor mistake, we will contact you, to resolve the issue)
- You can use any 3rd party libraries/frameworks (although you may need to
  explain why it's needed, and how it works internally)

---

### Score Board

```
+----------------------+----------------+---------------+
|         Name         |       OS       | Average score |
+----------------------+----------------+---------------+
| The biggest chad     | Windows        |            61 |
| Sage                 | Linux          |            89 |
| Optimizatron         | Linux          |           200 |
| Chionger             | Linux          |           301 |
| Leader of the memory | Linux          |           811 |
| Points Burner        | Linux          |          1061 |
| Combo Breaker        | Linux          |          1760 |
| Trix                 | MacOS          |          1824 |
| Hackeromatic         | Linux          |          1930 |
| Poseidon             | Linux          |          1943 |
| Tracer               | Linux          |          2106 |
| Bytes Maestro        | Linux          |          4132 |
| Leaky boi            | Cross-platform |          5700 |
| James May            | Cross-platform |         51132 |
+----------------------+----------------+---------------+
```

Disclaimer:
All the scores are measured on rather average laptop.
If you are comparing those scores - take them with a grain of salt, as different
computers may perform a little bit differently.

Good luck!
