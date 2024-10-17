# Manual

---

### Search


1. Search Commands:

- `File` OR `Session`, REQUIRED. eg: `--file={num}`, `--session={num}`, means re-use file or session by index number In reverse order;
- `Account`, Optional;
- `Balance`, Optional;
- `Budget`, Optional;
- `Register`, Optional;

2. Search Options:

- `Group` - Optional: `year`, `month`, `week`, `day`;
- `Recent` - Optional: `{num}year`, `{num}month`, `{num}week`, `{num}day`, {num} Is number of period;
- `Period` - Optional: `{yyyy-mm-dd}-{yyyy-mm-dd}`, means `{datetime_start}-{datetime_end}`;



### Config

- `Config`
  - `budget.alert`, eg: `--budget.alert=80`, options=range(0, 100);
  - `budget.postpone`, eg: `--budget.postpon=true`, options=[true, false];
  - `theme.font`, eg: `--theme.font=arial`, should have set font installed first;
  - `theme.color`, eg: `--theme.color=dark`, options=[dark, light, etc.], should have prepared first;
  - `generate.example`, eg: `--generate.template={file}`, will create default example file with set file path;


### Info

- `List`
  - `file`, eg: `--file={num}`, extracting history file path from log file, num.max=10, num.default=0, means no limit, example output:
    0: file_path
    1: file_path
    etc.;
  - `session`, eg: `--session={num}`, extract history session from log file, num.max=10, num.default=0, means no limit, example output:
    0: {session_0}
    1: {session_1}
    etc.;
