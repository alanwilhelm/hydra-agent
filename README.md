# hydra-agent
## This is the server agent for Hydra.
This agent collects information from and performs operations on the server. It works as a standalone binary, but is meant to be used on many servers at once and controlled by the Hydra REPL application over SSH.

### Warning
- This is an early prototype. Be careful.
- This has only been tested on OSX and late 2017 Ubuntu releases

### Requirements
- Use `rustup default nightly`
- OpenSSL `sudo apt-get install pkg-config libssl-dev`

### Todo:
- [x] Return basic system stats
- [ ] Return advanced system stats
- [x] Docker control/introspection
- [ ] Docker create container
- [ ] Postgres daemon control via systemd
- [ ] Nginx process control via systemd

###### Dev Todo
- [x] Return Cpu count, CPU speed, Os, Release
- [ ] Return disk space stats
- [ ] Return memory usage stats
- [ ] Struct for Error Response
