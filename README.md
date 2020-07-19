# hivemind

A simple distributed key-value store.

Project goals:
* single binary
* web interface for monitoring
* simple http interface for get/set
* partitioning


# Connection Summarized

Me: "Hey Server A, can I join you and your friends?"
Server A: "Sure, i'll consider you one of my peers"
Me: "Cool, i'll consider you one of my peers, what other peers do you know about?"
Server A: "I also know about Server B and Server C"
Me: "Hello Server B and C"
Server B: "Yo, i'll consider you one of my peers now"
Server C: "Whassup, i'll consider you one of my peers now too"
Server B: "Hey Server A and Server C, we have a new peer"
Server A: "I know"
Server C: "I know"
Server B" "Hey Server A and Server B, we have a new peer"
Server A: "yep"
Server B: "I know"