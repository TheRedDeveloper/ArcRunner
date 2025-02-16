# About ArcRunner

> [!CAUTION]
> Won't work for Power Toys Run

Arc.exe accepts no arguments. ArcRunner.exe enables you to run Arc with arguments, just like with Chrome, Firefox, etc. This is very useful for running Arc with [Flow Launcher](https://github.com/Flow-Launcher/Flow.Launcher).

**Just run `ArcRunner.exe`** and it will set itself up to be used by [Flow Launcher](https://github.com/Flow-Launcher/Flow.Launcher).

# Get ArcRunner

## Downloading
Download the latest `ArcRunner.exe` from [here](https://github.com/TheRedDeveloper/ArcRunner/releases/latest).

## Building
Clone the repository:
```batch
git clone https://github.com/TheRedDeveloper/ArcRunner
```

Build the project:
```batch
cargo build --release
```

Your ArcRunner binary is in `target/release/ArcRunner.exe`.

# Setup ArcRunner
> [!WARNING]  
> Arc needs to be set as the default browser.

Move `ArcRunner.exe` wherever you'd like.
> [!NOTE]  
> If you move `ArcRunner.exe` again, you will have to run it again.

**Just run ArcRunner.exe**, and it will set Arc as your Browser in Flow Launcher.
It just works!

> [!IMPORTANT]  
> `ArcRunner.exe` is NOT a virus. You can view and build the code yourself.
> Create an exception if your AntiVirus is overly aggressive and detects ArcRunner as an infected file.

# How to use ArcRunner
`ArcRunner.exe` supports `[url]` and `? [search query]` as an argument.

`? [search query]` uses Google to search, if you don't want to use Google simply change the code yourself or open a GitHub issue.
