Fleet Master
============

Fleet is a tool to collect information from different (remote) servers 
and applications through SSH.
 
Currently the only information providers are the builtin provider `fleet provide` 
and the provider for TYPO3 [fleet_typo3](https://github.com/cundd/fleet_typo3).

To get an overview of the available commands run:

```bash
fleet help
```


Building
--------

### macOS 10.10+

The dependency [libssh2](https://github.com/alexcrichton/ssh2-rs) links against OpenSSL and to compile it, it needs to 
find the OpenSSL headers. OpenSSL headers have been removed on macOS 10.10+, but can be installed via 
[Homebrew](https://brew.sh/):

```sh
brew install openssl
```

A compatible version of `cmake` also has to be installed:

```sh
brew install cmake
```


Configuration
-------------

Fleet expects a JSON file with server configurations. If `fleet.json` or 
`.fleet.json` exists in the current working directory it will be used. 
Otherwise the path to the configuration file has to be specified through 
the `-c` argument.

A simple configuration file may look like this:

```json
{
    "host-without-login-credentials": {
        "host": "host1.tld",
        "command": "path/to/the/provider/script",
        "username": "your-username"
    },
    "host-with-password": {
        "host": "host2.tld",
        "command": "path/to/the/provider/script",
        "username": "your-username",
        "password": "your-password"
    },
    "host-with-ssh-key": {
        "host": "host3.tld",
        "command": "path/to/the/provider/script",
        "username": "your-username",
        "private_key": "/your/home-dir/.ssh/id_rsa",
        "public_key": "/your/home-dir/.ssh/id_rsa.pub"
    },
    "host-with-custom-port": {
        "host": "host4.tld",
        "port": 2222,
        "command": "path/to/the/provider/script",
        "username": "your-username"
    }
}
```


If your SSH keys require a passphrase you have to specify it through the `PASSPHRASE` environment variable: 

```bash
PASSPHRASE=yourPassphrase fleet list
```


Todo
----

- ~~Make the port configuration optional~~
- Resolve the tilde `~` in the path to the SSH key files
- Support for ssh-agent
