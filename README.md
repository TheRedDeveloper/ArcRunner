# About ArcRunner

> [!CAUTION]
> Currently broken see [issue #1](https://github.com/TheRedDeveloper/ArcRunner/issues/1)

Arc.exe accepts no arguments. ArcRunner.exe enables you to run Arc with arguments, just like with Chrome, Firefox, etc.
This is very useful for running Arc with [PowerToys Run](https://github.com/microsoft/PowerToys) as it won't otherwise work.

# Get ArcRunner

## Downloading
Download the latest `ArcRunner.exe` from [here](https://github.com/TheRedDeveloper/ArcRunner/releases/latest).
Refer to [Setup ArcRunner](README.md#setup-arcrunner).

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
Refer to [Setup ArcRunner](README.md#setup-arcrunner).

# Setup ArcRunner
> [!WARNING]  
> Set Arc as the default browser before the setup process. If you don't do this, you might break your other browser.

Move `ArcRunner.exe` wherever you'd like.
> [!NOTE]  
> If you move `ArcRunner.exe` again, you will have to go through the setup process again.

Open a terminal where you moved `ArcRunner.exe` and run:
```batch
ArcRunner.exe install
```
> [!IMPORTANT]  
> `ArcRunner.exe` is NOT a virus. You can view and build the code yourself.
> Create an exception if your AntiVirus is overly aggressive and detects ArcRunner as an infected file.

If you installed ArcRunner successfully, you can now use Arc with [PowerToys Run](https://github.com/microsoft/PowerToys) and other applications.

# How to use ArcRunner
`ArcRunner.exe` supports `[url]` and `? [search query]` as an argument.

`? [search query]` uses Google to search, if you don't want to use Google simply change the code yourself or open a GitHub issue.