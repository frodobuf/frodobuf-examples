# calculator

This app shows an actor calling a Calculator capability provider, and the Calculator capability provider implementing a Calculator service (trait).


The `relay` actor acts like the Pinger actor in actor-to-actor. It receives an http request, and then generates a request to the capability provider. The capability provider's response is returned in the Relay's http response.

