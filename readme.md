# Basic chat
Provides basic chat functions:
- send messages
- fetch messages

## Components
- STP - custom string transfer protocol library above TCP.
- Chat server - server application, that holds messages and accepts incoming connections.
- Chat client - thin wrapper library around STP. Provides chat functions.
- Chat TUI - chat application with terminal interface. Uses chat client library.