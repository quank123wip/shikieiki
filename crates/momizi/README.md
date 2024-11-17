# Momizi 🌸🐺

> An autumn waterfall. I think the thing waterfalls convey the most is autumn.

Momizi is a small executer fits in judge tasks.

## Usage

``` bash
~ momizi --help
A small executer fits in judge tasks.

Usage: momizi [OPTIONS] <ARG>

Arguments:
  <ARG>
          The arg to execute

Options:
  -o, --output <OUTPUT>
          Decide where the executable will output
          
          [default: stdout]

          Possible values:
          - stdout: Assume the executable outputs in the STDOUT
          - file:   Assume the executable outputs in a file

  -m, --mode <MODE>
          Decide how to receive the result
          
          [default: compare]

          Possible values:
          - bool:    Receive the given result as boolean. (passed or not)
          - score:   Receive the given score by percentage
          - compare: Compare the result

      --log
          Outputs the log in stderr or not

  -f, --file <FILE>
          The file executable produces

  -a, --answer <ANSWER>
          The answer file for comparison

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Output

The momizi judger would give you a number in stdout, representing the score in precentage. Notice that this number could be a decimal or an integer.

## License

The code is licensed under GPL-v3.