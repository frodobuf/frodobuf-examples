

all:
	make -C http-server
	make -C actor-to-actor
	make -C calculator
	make -C echo

clean:
	make -C http-server clean
	make -C actor-to-actor clean
	make -C calculator clean
	make -C echo clean
