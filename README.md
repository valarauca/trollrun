trollrun
---

TrollRun is a simple program that runs many instances of [`troll`](https://topps.diku.dk/torbenm/troll.msp)
at the same time, collects their output, and provides a nice CSV style report.

### How to setup troll

For starter you will need the [`troll`](http://hjemmesider.diku.dk/~torbenm/Troll/). The source
code for it can be found [here as a .zip](http://hjemmesider.diku.dk/~torbenm/Troll/Troll.zip).

To compile the source you will need the [moscow ml compiler](https://mosml.org/). 

Once you have moscow-ml installed, you can compile troll via the shell scripts within the the
.zip, and play around.

### How to setup trollrun

You will need the rust-compiler, and possibly git to download & compile this tool.

### How to use trollrun

This program takes 1 argument which is a path to the configuration file which describes its run.

### How to configure trollrun

The input configuration is a `toml`, where almost all fields are optional.

The first structure is the `[troll]` structure which describes where the
troll executable can be found (assuming `troll` is not within your `$PATH`).

```toml
[troll]
path = "/path/to/troll/executable"
```

The next optional structure is the `[csv]` structure, it describes some aspects of the CSV.
Remeber all of this optional.

```toml
[csv]
path = "/path/to/output.csv"
seperator = ','
quote = '"'
precision = 12
zero_path = 3
flush_to_zero = 0.0001
```

1. `path` is not specified the tool will print to stdout.
2. `seperator` is not specified the tool will use `,`. If a non-ascii seperator is specified it will be ignore
3. `quote` is not specified the tool will use `"`". If a non-ascii quote is specified it will be ignored.
4. `precision` specifies how many decimal positions to serialize.
5. `zero_pad` specifies how many `0` to include in front of the number.
6. `flush_to_zero` specifies the minimum value the CSV should include, values smaller will be rounded to `0`.

The `[runs]` entry is the critical component as it describes what programs should executed (always in parallel).

```toml
[runs]
name1 = "/path/to/troll/program.t"
name2 = "/path/to/a/different/program.t"
```

The description is simply as free form `name` and `path` describe the header, and the program to be ran.

More complex runs can be given in the style

```toml
[runs]
name1 = "/path/to/troll/program.t"
name2 = "/path/to/a/different/program.t"
name3 = { path = "/path/to/complex/program.t", args = { rr1 = 1, hit = 3} }
```

The complex description allows for passing of CLI arguments to troll to provide values to un-initialized variables.

This allows for the same program to be executed in multiple different manners


