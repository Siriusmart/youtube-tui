# Invidious

Invidious is a free and open-source alternative frontend to YouTube. It is intended to be used as a lightweight and "privacy-respecting" alternative to the official YouTube website.

## How Invidious is used in youtube-tui

The TUI does not access YouTube using its official API, instead it uses the Invidious API to access videos - since it is much easier to use and does not require users to be logged using their API token.

By sending a request to one of their [API paths](https://docs.invidious.io/api/), it returns a JSON response containing information about videos, playlists, channels which can be understood by the program.

## YouTube is blocking Invidious

Referencing to [this GitHub issue](https://github.com/iv-org/invidious/issues/3822), YouTube has started trying to block public Invidious instances. So far some Invidious instances are blocked from accessing video information, the TUI displays an error when that happens, which may happen more and more frequently, and to more instances.

There are several ways to bypass this.

### Use smaller, or private instances

All public instances can be found in [api.invidious.io](https://api.invidious.io/), it is likely there are instances on the list which are not blocked from accessing YouTube videos. Change your `invidious_instance` option in [`main.yml`](./config/main.md) to the new instance address.

If you know people who host their own websites and services, it is possible that they already have their own private Invidious instance running. If that's not the case, try asking nicely for them to host an Invidious instance, it might work. (Stop being the introvert you are, go touch some grass and talk to humans)

### Self host a local instance

[Here](https://docs.invidious.io/installation/)'s the official installation guide, and [here](https://github.com/tmiland/Invidious-Updater)'s a cool install script.

> Note: the install script is a bit buggy and does not prompt you to press `y`, so it may appear that it is hanging. For that you can just look into `invidious_installer.log` to see if you are supposed to press `y`.

No more info for now.
