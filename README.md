# asroute

`asroute` is a CLI tool for parsing traceroute output to summarize AS's traversed.

Ever wondered how your packets get from home to Bhutan? I sometimes like to see it, not at the hop level, but the companies and their networks that carry my data. Or how much shorter the paths are if the site uses a CDN?

Uses Team Cymru's [Rust crate](https://docs.rs/cymrust/0.3.4/cymrust/) for the ASN to AS Name lookup 

## Installation

### Homebrew
```bash
$ brew install asroute
```

## Usage

```bash
$ traceroute -a www.bhutan.gov.bt | asroute
traceroute to bhutan.gov.bt (202.144.128.217), 64 hops max, 52 byte packets
-> AS0 (Reserved)
-> *
-> BRESNAN-33588, US
-> LIGHTOWER, US
-> BRESNAN-33588, US
-> CHARTER-20115, US
-> TELIANET Telia Carrier, EU
-> *
-> NTT-COMMUNICATIONS-2914, US
-> DRUKNET-AS DrukNet ISP, BT
-> BTTELECOM-AS-AP Bhutan Telecom Ltd, BT
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Todos

- [ ] -h with usage
- [ ] progress indicator
- [ ] mode to execute traceroute rather than parsing output
- [x] brew install

## Notes

The [lft](https://en.wikipedia.org/wiki/Layer_four_traceroute) tool contains similar functionality and is a feature-rich layer 4 tracing tool (although requires sudo). If you're doing serious network analysis, that is probably more for you. This tool was more to scratch an itch for doing a Rust CLI and seeing how easy (very!) it is to distribute via homebrew.

## License
[MIT](https://choosealicense.com/licenses/mit/)
