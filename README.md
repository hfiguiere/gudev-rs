Rust bindings for libgudev.

Essentially for the GUdev::Client class that allow listening to udev
events in a Gtk application.

To generate the bindings, do:

````
python3 generator.py --embed-docs
````

Then you can build them with:

````
cargo build
````


Contributors
------------

Maintainer:

- Hubert Figuière <hub@figuiere.net>

Contributors:

- Bilal Elmoussaoui
- Moritz Maxeiner
