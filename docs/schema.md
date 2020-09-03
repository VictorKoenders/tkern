# Schema

Schemas are TKern's way of interfacing with the world. Several schemas are defined below.

Schemas start with `<name>:`. Common schemas are `file:/`, `https://`, `ftp://`.

## File

The `file:` schema indicates a file. Opening a file schema will turn the current window into a file browser (if it's a directory) or a file viewer (if it's a file).
Files can be either relative: `file:image.png` or absolute: `file:/app/calculator/calculator`

## Web

The `web:`, `http://` and `https://` schemas indicate a website. Opening a web schema will turn the current window into a browser.

When using `web:`, `https://` is implied. Users can override this by using `http://` directly.

## Config

The `config:` and `settings:` schema indicate a page where the user can change the configuration of an installed application

## System

The `system:` schema indicates an interaction with the system, e.g. `system:cpu_info` shows the info of the cpus on the system.

## Custom

Users can create their own schemas. Think of `games:` which would show a list of installed games, and each game would be their own url like `games:factorio`.

## Schema lookup

When an unknown schema is entered in the url, an schema lookup is triggered. Apps can register themselves for this lookup, and get called whenever the lookup happens. Apps will run a custom script that returns how relevant that lookup is for them, which will be suggested to the user.

e.g. when the user types `facebook.com`, the `web` app knows this is a website and can indicate that it wants to launch `web:facebook.com` with a higher priority than the calculator app. The calculator app can see `2+2` and indicate it wants to launch `calc:2+2`.
