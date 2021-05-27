
This is a "port" of the wasmcloud actor-to-actor demo to frodobuf.

The interface is defined in ping-interface/ping.widl.

The pinger actor implements HttpServer, defined in [the http-server
exapmle](./http-server), and Actor interface, defined in
frodobuf/interfaces/system.

The ponger actor implements the Ponger interface, defined
in ping-interfaces/ping.widl, and the Actor interface.


# Running

To build and run, you will need both `wash` and `wasmcloud` in your
path. 

From this folder, type `make` to build, and `make run` to run the
wasmcloud host, install the two actors and http server capability
provider, and set up the link between pinger and the http server
provider.

To trigger the conversation, type 
`curl localhost:8080/x`, which sends an HttpRequest to pinger.
Pinger then sends a Ping to ponger, who responds 
with another string message. Pinger returns the response (that it
received from ponger) to the http client, your curl command.

