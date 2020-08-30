# asroute

`asroute` is a CLI tool for parsing traceroute output to summarize AS's traversed.

Ever wondered how your packets get from home to Bhutan? I sometimes like to see it, not at the hop level, but the companies and their networks that carry my data. Or how much shorter the paths are if the site uses a CDN?

Uses Team Cymru's [Rust crate](https://docs.rs/cymrust/0.3.4/cymrust/) for the ASN to AS Name lookup 

## Installation

- TODO: brew
- TODO: cargo

```bash
brew install...
```

## Usage

```bash
$ traceroute -a www.bhutan.gov.bt | asroute
traceroute to bhutan.gov.bt (202.144.128.217), 64 hops max, 52 byte packets
-> AS0 (Reserved)
-> COMCAST-7922, US
-> CMCS, US
-> COMCAST-7922, US
-> *
-> *
-> NTT-COMMUNICATIONS-2914, US
-> DRUKNET-AS DrukNet ISP, BT
-> BTTELECOM-AS-AP Bhutan Telecom Ltd, BT
-> *
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Todos

- [ ] -h with usage
- [ ] progress indicator
- [ ] mode to execute traceroute rather than parsing output
- [ ] brew install

## License
[MIT](https://choosealicense.com/licenses/mit/)