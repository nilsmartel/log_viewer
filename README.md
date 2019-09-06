# log_viewer

simple cli to easier disect the contend of logs.

# Purpose (for now)

Read line by line from stdin (Streaming)
if a line is in `json` format, color code it based on `.level` (might be `info` etc.) and print it
otherwise just print it (possibly in grey)?

# Considerations

what might be useful:
    -   1.) simple arguments to highlight only certain log levels (output everything else in grey)
    -   2.) simple arguments to only output certain log levels
    -   3.) maybe default log color for neutral lines (default black, possibly grey/lightgrey)?

How this might look in practice:
- `kubectl logs ... | log_viewer --highlight info danger`
- `kubectl logs ... | log_viewer --select info`
- `kubectl logs ... | log_viewer --background lightgrey`
