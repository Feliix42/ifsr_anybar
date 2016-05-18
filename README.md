# ifsr_anybar

This is a small script that checks the [buerostatus](https://github.com/fsr/buerostatus) from @fsr and changes the color of an Anybar dot to _green_ if someone is there and to _red_ if the office is closed.

## Installation

You will need [AnyBar](https://github.com/tonsky/AnyBar) installed in order to use this.

Follow these steps to get `ifsr_anybar` up and running on your computer and get status updates every minute:

1. Clone the repository:  
  ```
  $ git clone
  ```

2. Build the project:
  ```
  cd ifsr_anybar
  cargo build --release
  ```
  Note that you might have to set the compiler flags for OpenSSL. You can find them on OS X with `brew info openssl`.

3. _(optional)_ Move the compiled binary to `/usr/local/bin` so that you can remove the cloned repository afterwards.

4. Add a cronjob to update the status regularly. Type
  ```
  $ crontab -e
  ```
  And add the following line at the end of the file:
  ```
  * * * * * /path/to/the/binary/ifsr_anybar [Port]
  ```
  You can omit the Port if you are using the standard AnyBar port _(1738)_.

## Roadmap

- [x] Support for non-standard Ports (`-p flag`?)

## License
This work is licensed under the **MIT License**. See `LICENSE` for more information.

Portions of this work are shamelessly stolen from urschrei's [rust_anybar](https://github.com/urschrei/rust_anybar).
