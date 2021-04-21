workflowy-cli
==============

`workflowy-cli` is a experimental tool that export data from the awesome [WorkFlowy](https://workflowy.com) web-based
outliner to various formats.

It's written in Rust.

I'm using this project to experiment with this language, take this into consideration if you choose to give it a try.

Any feedback would be very appreciated, please contact if you have any advice to improve this tool or help me climbing
up the Rust' steep learning curve.

![](.doc/demo.gif?raw=true)

## How to build it?

### Install cargo

You need to build it yourself, I don't want to manage binary download for now.

Nevertheless, it's straightforward to do so, just follow

the page [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) from
the [Cargo Book](https://doc.rust-lang.org/cargo).

`cargo` should be then available.try

    $ cargo --version
    cargo 1.49.0 (d00d64df9 2020-12-05)

### Clone the repository

    $ git clone ... ...

### Build with cargo

    $ cargo build --release
    ...
    Finished release [optimized] target(s) in 9.42s

The binary built is available in `target/release/workflowy-cli`.

As it is self-contained, it can be invoked directly or copied somewhere convenient to you (to be available on your path
for instance).

## Connect to WorkFlowy

`workflowy-cli` needs a way to access your WorkFlowy account to then export data. As WorkFlowy does not officially
support API to do so, you need to provide your session identifier from the cookie WorkFlowy stored for you in your
browser.

There are many ways to do this, I like
the [EditThisCookie](https://chrome.google.com/webstore/detail/editthiscookie/fngmhnnpilhplaeedifhccceomclgfbg)
extension for Google Chrome.

![](.doc/sessionid.png#1?raw=true)

Copy this value of the cookie `sessionid` into the clipboard. It will then be stored locally in your home folder by
invoking the following command.

    $ workflowy-cli auth
    sessionid=<paste here the value and press Enter>

## Export data from WorkFlowy

For the time being, only exports targeting [Anki](https://apps.ankiweb.net) are available. Anki is a free and
open-source flashcard program to help memorize things.

### Anki

Consider this data layout in WorkFlowy representing definitions you eventually want to convert into Anki cards.

![](.doc/vocabulary.png#1?raw=true)

`workflowy-cli` can be invoked to extract this data into a csv file that Anki will recognize.

    $ workflowy-cli export -f anki-dict -p meaning -p tag -o deck.csv 'https://workflowy.com/#/<id>'

- `export` use the export command to export data
- `-p meaning -p tag` select any children starting with the words `meaning` or `tag`, at least one prefix should be
  specified
- `-o deck.csv` export data to deck.csv on disk
- `'https://workflowy.com/#/<id>'` internal link of the root, i.e. the item with the `Vocabulary` text

The file `deck.csv` can then be used to import cards into Anki.
