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
`WorldServer <http://dev.worlds.net/private/GammaDocs/WorldServer.html>`_
protocol in `Rust <https://www.rust-lang.org/>`_.

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

Documentation about the entire Whirlsplash ecosystem can be found
`here <https://whirlsplash.org/docs/>`_.

If you are looking for explicit documentation of only Whirl's source code, you
must generate them yourself be cloning the repository and running the Cargo
subcommand: :code:`cargo +nightly doc --open --document-private-items --no-deps`.

Known Issues
------------

- https://github.com/Whirlsplash/whirl/issues/2

Contributing
------------

Please reference the `contribution guidelines <./CONTRIBUTING.md>`_ of this repository.

Development Dependencies
------------------------

Required
~~~~~~~~

- `Rust <https://www.rust-lang.org/>`_
- `diesel\_cli <https://crates.io/crates/diesel_cli>`_
- `cargo-make <https://github.com/sagiegurari/cargo-make>`_

Optional
~~~~~~~~

- `cargo-watch <https://crates.io/crates/cargo-watch>`_
- `sqlfluff <https://github.com/sqlfluff/sqlfluff>`_
- `Valgrind <https://www.valgrind.org/>`_

*These development dependencies (excluding sqlfluff) will automatically be satisfied if you are using the Nix shell
configuration as provided.*

Benchmarking
------------

For the time being; benchmarking is only available for \*nix-based environments as the benchmarking
utility currently in place, `Iai <https://github.com/bheisler/iai>`_, has a hard dependency on
`Valgrind <https://github.com/bheisler/iai>`_ which is only accessible within \*nix-based
environments.

More so, the current benchmarks which have been set-up do not cover the full extent of Whirl, but a
small portion of the crates which make up Whirl as a whole. In the future, benchmarks may be
expanded upon.

To execute the available benchmarks, use the following Cargo subcommand;

.. code-block:: shell

  $ cargo bench

License
~~~~~~~

`GNU General Public License v3.0 <./LICENSE>`_
