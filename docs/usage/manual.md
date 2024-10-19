# Manual

---

### Search


1. Search Commands:

- `File`, Optional. eg: `--file={num}` means re-use file by index number In reverse order; or `--file={file_path}` directly;
- `Account`, Optional;
- `Balance`, Optional;
- `Budget`, Optional;
- `Register`, Optional;

2. Search Options:

- `Group` - Optional: `year`, `month`, `week`, `day`;
- `Recent` - Optional: `{num}year`, `{num}month`, `{num}week`, `{num}day`, {num} Is number of time frame, default|minimum to num=1;
- `Period` - Optional: `{yyyy-mm-dd}-{yyyy-mm-dd}`, means `{datetime_start}-{datetime_end}`;



### Info

- `List`
  - Extracting list of different history file path from log file, maximum limit to 10, display in reverse time order;
    0: latest_used_time: file_path
    1: latest_used_time: file_path
    2: latest_used_time: file_path
    3: latest_used_time: file_path
    . . . . . . 
    9: latest_used_time: file_path


- `Example`
  - `file`, eg: `file={file_path}`, will create default example file with defined file path, or to executable path if this argument not present, eg: `<ledger.exe path>/example.yaml`;
