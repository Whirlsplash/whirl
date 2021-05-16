.. raw:: html

  <p align="center">
    <a href="https://github.com/Whirlsplash/whirl">
      <img
        src="https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png"
        alt="Whirl"
        width="220">
    </a>
  </p>
  <h1 align="center">The Open-Source WorldServer.</h1>

  <p align="center">

.. image:: https://img.shields.io/discord/821938182274154506
  :alt: Discord
  :target: https://discord.com/invite/8hn6padWF6

.. image:: https://www.codefactor.io/repository/github/whirlsplash/whirl/badge
  :alt: CodeFactor
  :target: https://www.codefactor.io/repository/github/whirlsplash/whirl

.. image:: https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg
  :alt: Say Thanks
  :target: https://saythanks.io/to/fuwnzy@gmail.com

.. image:: https://img.shields.io/github/license/Whirlsplash/whirl
  :alt: License
  :target: ./LICENSE

.. raw:: html

  </p>

Whirl, an open-source implementation of the
`WorldServer <http://dev.worlds.net/private/GammaDocs/WorldServer.html>`__
protocol in `Rust <https://www.rust-lang.org/>`__.

Purpose
-------

Two *main* reasons;

1. Worlds' official servers are old, slowly deteriorating, and *probably unmaintained*.
2. Worlds is **old**. The shutdown of the servers is inevitable.

**TLDR**: To keep Worlds' legacy going for future generations.

Usage
-----

Use in production environments is **not** recommended as this project is currently under heavy
development.

As stability increases, periodic updates regarding production use will be released (via Discord or
blog).

Documentation
-------------

https://whirlsplash.org/docs/

Known Issues
------------

-  https://github.com/Whirlsplash/whirl/issues/2

Contributing
------------

Please reference the `contribution guidelines <./CONTRIBUTING.md>`__ of this repository.

Development Dependencies
------------------------

Required
~~~~~~~~

-  `diesel\_cli <https://crates.io/crates/diesel_cli>`__
-  `cargo-make <https://github.com/sagiegurari/cargo-make>`__

Optional
~~~~~~~~

-  `cargo-watch <https://crates.io/crates/cargo-watch>`__

*These development dependencies will automatically be satisfied if you are using the Nix shell
configuration as provided.*

License
~~~~~~~

`GNU General Public License v3.0 <./LICENSE>`__