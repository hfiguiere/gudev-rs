Rust bindings for libgudev.

Essentially for the GUdev::Client class that allow listening to udev
events in a Gtk application.

To generate the bindings, do:

````
make gir
````

Then you can build them with:

````
cargo build
````


