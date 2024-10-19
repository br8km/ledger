
# Features

---

### Main.Functions

- Check Account File Format, Load Config;
- Search|List;
- Statistics|Chart;
- Interactive;
- Instant.Autocomplete;


### General.Features

- Cross-Platform;
- All Language & Currency;
- Input File Type: `yaml`;
- Pretty Reporting;
- Pretty Logging;


### Usage 

- Options

- Commands
    - `ACCOUNT`
    - `BALANCE`
    - `BUDGET`
    - `REGISTER`
    - `FILE`
        - `path`
        - `{0-9}` - history.file.index
    
- Options - `REGISTER`
    - `groupby`
        - `year`
        - `month`
        - `week`
        - `day`
    - `recent`
        - `{2}year`
        - `{3}month`
        - `{6}week`
        - `{15}day`
    - `period`
        - `20240101-20241001`

- Options.TODO
    - `history-cmd` - history commands
        - `{0-9}`
    - `history-file` - history files
        - `{0-9}`

- Custom.Options.from.File
    - `decimal`, default `6`; if crypto, set to `12`