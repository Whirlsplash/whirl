<p align="center">
  <a href="https://github.com/Whirlsplash/whirl">
    <img src="https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png" alt="Whirl icon" width="220" />
  </a>
</p>
<h1 align="center">
  Whirl, an open-source <a href="http://dev.worlds.net/private/GammaDocs/WorldServer.html">WorldServer</a> implementation in <a href="https://www.rust-lang.org/">Rust</a>.
</h1>

<p align="center">
  <a href="https://discord.com/invite/8hn6padWF6" title="Discord">
    <img src="https://img.shields.io/discord/821938182274154506">
  </a>
  <a href="./LICENSE" title="License">
    <img src="https://img.shields.io/github/license/Whirlsplash/whirl">
  </a>
</p>

## Purpose
Two *main* reasons;
1. Worlds' official servers are old, slowly deteriorating, and *probably unmaintained*.
2. Worlds is **old**. The shutdown of the servers is inevitable.

TLDR: To keep Worlds' legacy going for future generations.

## Usage
Use in production environments is not recommended as this project is a work-in-progress. As we begin to become more stable and add new features, we will release periodic updates regarding production use.

## Documentation
[https://www.whirlsplash.org](https://www.whirlsplash.org)

## Development
1. Navigate to your `/etc/hosts` file, on Windows, this usually is located at `C:\Windows\System32\drivers\etc\hosts`.
2. Add these rules;
```
0.0.0.0				www.3dcd.com
0.0.0.0				test.3dcd.com
```
3. Build and run the Cargo project.
4. Profit.

## Contributing
Please reference the [contribution guidelines](./CONTRIBUTING.md) of this repository.

### License
[GNU General Public License v3.0](./LICENSE)
