# Keyport: A SSH utility program
Simplifies adding and removing SSH keys on UNIX like systems.
Requires `openssh` installed on your system.

## Installing

## Basic usage
### Adding an existing key
```bash
$ keyport add <name> # i.e. id_ed25519, id_rsa, ...
```

### Removing an added key
```bash
$ keyport remove <name>
```

# Roadmap
- [ ] Allow for password protection 
- [ ] Support custom editors
- [ ] Key validation (when adding)
- [ ] List command to list active and inactive keys
- [ ] Generating new keys
