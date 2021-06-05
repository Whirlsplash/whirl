Contribution Guidelines
=======================

This document isn't very well detailed at the moment...

Guidelines
----------

1. Make sure your PR compiles. If the GitHub action "Rust ✅/ 🚫" fails, your PR
   will be declined.
2. Make sure your commit changes something other than just formatting.
3. If you change updates any of the :code:`Cargo.toml` files, make sure to
   `install <https://github.com/est31/cargo-udeps#installation>`_ and run
   :code:`cargo +nightly udeps` to ensure there aren't any unused dependencies.
