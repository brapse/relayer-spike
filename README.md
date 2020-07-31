Relyaer Spike
=============

The goal is to outline the relayer logic as an object graph that can
abstract over concurrency and IO.

## Objects

LocalClient
    * Light clients embeded in the relayer

RemoteClient
    * Can be updated
        * Updating means producing update dragrams

Chain
    * Can be queried
    * Materialize (cache) state from the chain
    * produce subcriptions (event streams)
        * Can produce route events to different subscriptions
    * Have a RemoteClient 
    * can be sent Packets
    * Have clients?

Subscriptions (event Monitor)
    * Produce events
    * Different events can produce different 

Connections
    * ICS3
    * Required two chains
    * require a specific set of events
    * Are stateful
    * Materialized view on state of two chains
    * Performs handshake algorithm

Channels
    * Requires a connection
    * Materialized view of on-chain state
    * can timeout?
        * Does it require upkeep?

Packets
    * Are the product of events and queries
    * Batch multiple datagrams together

## Testing Regimen
* RemoteClient
    * With valid full node
    * With invalid full node (verification fails)
* Connection setup
    COnnection handshake as the function of two chain states
    * With out of date client
    * Channel setup
* Channel Setup
    * Function of chain states
* Packet construction
    * As the function of a two chain states

## Stage 1: Synchronous abstractions:
    * Chain, Subscription abstraction
    * Sketch dependency graph
    * Connection construction
    * Channel consutrction

## Stage 2: Runtime
    * produce relayer consutrciton with handler facades for concurrent runtimes

## Notes Readiing Libra
* Using channels to communicate between threads makes sense 
* Handlers should be synchronous interfaces as implemented traits and not raw channels to
  provide consistent testing surface areas
* proliferating channels makes reasoning about lifecycle management more
  complicated than it need be. Tasks should be modeled objects who
  communicate lifecycle expectations contractually through synchronous
  methods.

