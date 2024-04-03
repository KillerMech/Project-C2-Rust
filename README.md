<H1>Project C2 Rust</H1>
<H2>This is my capstone project</H2>

For this project it is recommended that you have rustup installed rather than
basic Rust. rustup is a Rust foundation project that allows you to manage
multiple versions of Rust on your system. This will also allow you to use
cross which is a tool that makes it easy to cross compile the agent to other
platforms.

<H2>File Structure Overview</H2>
All Rust source code can be found in the src directory. The main.rs file is
simply for launching the server. The server.rs rile is where most logic takes
place and the server.rs file calls on lib.rs and file_handler.rs.

<H2>How to Run</H2>
To run the server, simply run the following command in the root directory of
the project:
```
cargo run
```

This will build and run the project. If you would like to build the project
in a way that produces a binary, you can run the following command:
```
cargo build
```
The functions of the server are self explanitory once run. For the web page,
it has already been built and exists in the root directory of the project. The
source code for the web page can be found in the client_c2_rust directory. That
directory has it's own build and run instructions. Vue uses npm and so you will
need to install npm to build and test the web page if you wish to make changes.
If you do not wish to make changes, then you made ignore the client_c2_rust as
the release version of the client is already built and distributed with the
project.

<H2>Building the Agent</H2>
The agent is found in the rust_agent directory. To build the agent, you will
need to have cross installed. To install cross, run the following command:
```
cargo install cross
```
Once cross is installed, you can build the agent with the following command:
```
cross build --target <target>
```
Where <target> is the target platform you wish to build the agent for. To view
target platform options enter the command:
```
rustup target list
```
