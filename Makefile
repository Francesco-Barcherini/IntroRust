all: server client

server: server.rs
	rustc server.rs

client: client.rs
	rustc client.rs

clean:
	rm -f server client
	
	

