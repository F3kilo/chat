# Basic chat
Provides basic chat functions:
- create room
- send messages to room
- fetch room's messages

## Components
- STP - custom string transfer protocol library above TCP.
- Chat server - server application, that holds messages and accepts incoming connections.
- Chat client - thin wrapper library around STP. Provides chat functions.
- Chat TUI - chat application with terminal interface. Uses chat client library.