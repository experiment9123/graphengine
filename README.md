
illustration of an templated "graph engine", a collection describing a graph with update rules , with a niave single threaded unoptimised implementation.
intended to faciliate experiments with spiking neural nets.
"conways game of life" implemented using this engine.
the engine takes node types with member functions for:
- generating messages along edge connections to other nodes, 
- receiving these message
- an internal upate to perform once all these messages are received

The edge types also have an update step

the intention is to decouple node behaviour from a later optimised parallel implementation



