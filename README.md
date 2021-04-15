illustration of an templated "graph engine", a collection class describing a graph with update rules , with a naive single threaded unoptimised implementation.
intended to faciliate experiments with spiking neural nets.
"conways game of life" implemented as an example, by setting up a grid of nodes with 8 neighbour links, displayed using SDL2

the engine takes node types with member functions for:
- generating messages along edge connections to other nodes, 
- receiving these message
- an internal upate to perform once all these messages are received

The edge types also have an update step (where learning rules would go)

the intention is to decouple node behaviour from a later optimised parallel implementation



![](/IMG_3818.jpeg)
![](/screenshot.png)

