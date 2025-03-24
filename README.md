# Keyport: A SSH utility program
Simplifies adding and removing SSH keys on UNIX like systems.
Requires `openssh` installed on your system.

## Disclaimer
This is a rather hacky solution at the moment, so things might not work as expected.

## Installing

## Basic usage
When specifying only the name of a key, keyport will assume you mean a key in `~/.ssh/`.
- `keyport show id_rsa` will show the key `~/.ssh/id_rsa.pub`
- `keyport remove ./my_key` will remove the `my_key` key in the current directory.
- `keyport add /path/to/key` will let you add the key at the specified path.

### Adding an already generated key
```bash
$ keyport add <FILE> # i.e. id_ed25519, id_rsa, ...
```

### Removing an added key
```bash
$ keyport remove <FILE>
```

### Show a public key
```bash
$ keyport show <FILE>
```

### Set the password of a key
```bash
$ keyport set-password <FILE>
```

# TODOs
- [X] Show command to show public keys
- [X] Allow for password protection 
- [X] Support custom editors
- [ ] Key validation (when adding)
- [ ] List command to list active and inactive keys
- [ ] Generating new keys
- [ ] Open temporary files for the keys, validate them and only if everything is fine write them to the ssh directory.
- [ ] Improve the error handling
