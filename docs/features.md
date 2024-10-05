
# Features

- Command Line Tool With Cross-Platform Support;
- All Language & Currency Support;
- Smart Argument Parsing, No Ordering;
- Sub-Command
    - HELP
    - VERSION
    - ACCOUNT
    - BALANCE
    - REGISTER
        - `--groupby=year`
        - `--groupby=month`
        - `--groupby=week`

- Sub-Command.Advanced
    - period ???
        - [first|last]{n}[year|month|week|day]
            - `--year=2024`
            - `--month=202406`
            - `--week=20240603`
            - `--day=2`
            - `--recent=week|month|year`
            - `--last=7{d|w|y}` meaning last 7 days


        - [start|end]
    - history.file.index
        - eg: `--file={0-9}`, OR Abbr as: `-f {0-9}`

- Custom Color Theme Pretty Table Reporting;
- Support Input File Type: `yaml`;
