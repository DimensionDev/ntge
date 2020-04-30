# Not That Good Encryption (NTGE)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FDimensionDev%2Fntge.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FDimensionDev%2Fntge?ref=badge_shield)


Design doc: [Google Doc](https://docs.google.com/document/d/1rI3G_YtyeDMQmRI4D7oIdvdhjY4rfBeBA6c7aXybJhU/edit?usp=sharing) (Comments and suggestions are welcomed!)

Not That Good Encryption is a general-purpose rust-based encryption tool. Our main goal is to learn rust in a project-based way for a real on-hand practice for the team. We are also willing to integrate this tool/lib to our new encryption input method mobile app. If our implementation works well enough, we will definitely give it a try.

## Setup
```bash
$ cargo install --force cbindgen
```

## cbindgen
Generate header manually

```bash
$ cd ./ntge-core
$ touch build.rs
$ cargo build --features cbindgen-enable
```

## Acknowledgements
- [rage](https://github.com/str4d/rage)

## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FDimensionDev%2Fntge.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FDimensionDev%2Fntge?ref=badge_large)