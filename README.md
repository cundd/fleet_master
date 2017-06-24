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
        "port": "22",
        "command": "path/to/the/provider/script",
        "username": "your-username"
    },
    "host-with-password": {
        "host": "host2.tld",
        "port": "22",
        "command": "path/to/the/provider/script",
        "username": "your-username",
        "password": "your-password"
    },
    "host-with-ssh-key": {
        "host": "host3.tld",
        "port": "22",
        "command": "path/to/the/provider/script",
        "username": "your-username",
        "private_key": "/your/home-dir/.ssh/id_rsa",
        "public_key": "/your/home-dir/.ssh/id_rsa.pub"
    }
}
```


If your SSH keys require a passphrase you have to specify it through the `PASSPHRASE` environment variable: 

```bash
PASSPHRASE=yourPassphrase fleet list
```


Todo
----

- Make the port configuration optional
- Resolve the tilde `~` in the path to the SSH key files
- Support for ssh-agent
