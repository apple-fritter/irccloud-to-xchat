# irccloud-to-xchat

This is a Rust program that converts IRC logs from the IRCcloud format to the XChat format. It uses the `zip` crate to extract the logs from a ZIP archive and write them to separate log files.

I have also formed a program to convert IRCCloud logs to the [Weechat](https://github.com/apple-fritter/logconvert.irccloud-to-weechat) format.

## Installation

1. Install Rust and Cargo on your system.
2. Clone this repository: `git clone https://github.com/yourusername/my-irccloud-to-xchat-converter.git`
3. Navigate to the repository: `cd my-irccloud-to-xchat-converter`
4. Run the program: `cargo run`

## Usage

The program takes two arguments: the path to the input ZIP archive and the path to the output directory. Here's an example:

```bash
cargo run --release -- <input_zip> <output_dir>
```

Replace `<input_zip>` with the path to your input ZIP archive and `<output_dir>` with the path to the directory where you want to save the output log files.

## IRC Meta:

### WeeChat
- [weechat.ban-evasion-detection](https://github.com/apple-fritter/weechat.ban-evasion-detection): Detect and prevent ban evasion. Python.
- [weechat.typo-aggregator](https://github.com/apple-fritter/weechat.typo-aggregator): Records misspelled words in a TSV (tab-separated values) file. Python.
- [weechat.whois-aggregator](https://github.com/apple-fritter/weechat.whois-aggregator): Aggregate whois data in a rolling CSV file. Python.
- [weechat.youtube-info](https://github.com/apple-fritter/weechat.youtube-info): Extract video information from a YouTube URL and post it back to the channel. Python.

### IRCcloud
- [irccloud-to-weechat](https://github.com/apple-fritter/irccloud-to-weechat): Convert IRC logs from the IRCcloud format to the Weechat format. Rust.
- [irccloud-to-xchat](https://github.com/apple-fritter/irccloud-to-xchat): Convert IRC logs from the IRCcloud format to the XChat format. Rust.

### X-Chat
- [xchat.channel-moderation](https://github.com/apple-fritter/xchat.channel-moderation): Moderate an IRC channel. Python.
- [doppelganger](https://github.com/apple-fritter/doppelganger): X-Chat mIRC imposter. Fingerprint subversion. Python bundle.

### Other:
- [driftwood](https://github.com/apple-fritter/driftwood): A unified IRC log format defined. Written in Rust.

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
