.. raw:: html

  <p align="center">
    <a href="https://github.com/Whirlsplash/whirl">
      <img
        src="https://raw.githubusercontent.com/Whirlsplash/assets/master/Whirl.png"
        alt="Whirl"
        width="200">
    </a>
  </p>
  <h1 align="center">Whirl: The Open-Source WorldServer</h1>
  <h3 align="center">A Worlds United</h3>

  <br>

  <p align="center">

.. raw:: html

  </p>

Whirl is an open-source implementation of `Worlds <https://www.worlds.com/>`_'
`WorldServer <http://dev.worlds.net/private/GammaDocs/WorldServer.html>`_
protocol, written in the modern and safe `Rust <https://www.rust-lang.org/>`_.

Purpose
-------

Two *main* reasons;

1. Worlds' official servers are old, slowly deteriorating, and *probably* unmaintained.
2. Worlds is **old**; the shutdown of the servers is inevitable.

**TLDR**: To keep Worlds' legacy alive for future generations.

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
may generate them yourself be cloning the repository and running the `doc` Cargo
subcommand:

.. code-block:: shell

  $ cargo +nightly doc --open --document-private-items --no-deps # or `cargo make gen-docs`

Development Dependencies
------------------------

Required
~~~~~~~~

- `Rust <https://www.rust-lang.org/>`_ — The backbone of it all.
- `diesel\_cli <https://crates.io/crates/diesel_cli>`_ — Database migrations, setup, et cetera
- `cargo-make <https://github.com/sagiegurari/cargo-make>`_ — Cross-platform
  `make <https://www.gnu.org/software/make/>`_ substitute

Optional
~~~~~~~~

- `cargo-watch <https://crates.io/crates/cargo-watch>`_ — Recompilation on file-system changes
- `sqlfluff <https://github.com/sqlfluff/sqlfluff>`_ — SQL linting
- `Valgrind <https://www.valgrind.org/>`_ — Benchmarking

Licence
~~~~~~~

This project is licensed with the `GNU General Public License v3.0 <./LICENSE>`_.
