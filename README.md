# Keyport: A SSH utility program
Simplifies adding and removing SSH keys on UNIX like systems.
Requires `openssh` installed on your system.

## Disclaimer
This is a rather hacky solution at the moment, so things might not work as expected.

## Installing

## Basic usage
### Adding an already generated key
```bash
$ keyport add <name> # i.e. id_ed25519, id_rsa, ...
```

### Removing an added key
```bash
$ keyport remove <name>
```

### Show a public key
```bash
$ keyport show <name>
```

# TODOs
- [X] Show command to show public keys
- [ ] Allow for password protection 
- [ ] Support custom editors
- [ ] Key validation (when adding)
- [ ] List command to list active and inactive keys
- [ ] Generating new keys
- [ ] Open temporary files for the keys, validate them and only if everything is fine write them to the ssh directory.
