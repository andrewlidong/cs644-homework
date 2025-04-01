(★) What function does a DNS lookup to turn a domain name into an IP address?

getaddrinfo()

(★) What's the difference between bind and listen?

Bind is for binding a socket to a specific port and IP address.
Listen is for listening for incoming connections on a socket.  

(★) What flags do you pass to request a TCP/IP connection?

AF_INET, SOCK_STREAM

(★★) Final project (database): Let's provide a proper client interface to the database. Have the main process listen for connections (you can decide whether you want to do TCP or local Unix) and allow querying the database. You can decide what the protocol looks like; a simple one might have commands like get <key>\n and set <key> <value>\n. Write a client program that provides a nice command-line interface to send commands to the database.

(★★) Final project (web server): It's finally time to make a proper web server! Use the socket API to listen for TCP connections. You can fork off a child process to handle each connection, or wait until next week when we learn about multithreading. You can make up a simple TCP-based protocol (e.g., client sends hello server\n, server sends hello client\n back) and test it with telnet, or if you're ambitious you can implement HTTP on the server side – either yourself or using an existing library.

(★★) How can a server access its client's network address?

getpeername()

(★★) What happens if a client calls connect before the server has called bind? What about after the server has called bind, but before it has called listen? Write a program to find out.

If the client calls connect before the server has called bind, the connection will fail.
If the server has called bind, but before it has called listen, the connection will fail.

(★★) If I open a socket with SOCK_DATAGRAM, will recv always return a single packet at a time? What happens if the buffer is too small to fit the packet? Write a program to demonstrate what happens.

Recv will always return a single packet at a time.  If the buffer is too small to fit the packet, the packet will be truncated and there will be no way to get the rest of the packet.  

(★★) What's the difference between close and shutdown? Can you write a program that shows them behaving differently?

Close is used to close a socket.  Shutdown is used to close a socket in a specific direction.  You can shutdown the read, write, or both halves of the socket.  

(★★★) Use epoll to write a single-threaded server that can simultaneously handle multiple connections.

import socket
import select

def create_server(host='localhost', port=8080):
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((host, port))
    server_socket.listen(5)
    return server_socket

def handle_connection(client_socket):
    request = client_socket.recv(4096)
    print(f"Received request: {request}")
    client_socket.sendall(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Hello, World!</h1></body></html>")
    client_socket.close()

def main():
    server_socket = create_server()
    print(f"Server listening on {host}:{port}")

    try:
        while True:
            client_socket, addr = server_socket.accept()
            print(f"Accepted connection from {addr}")
            handle_connection(client_socket)
    except KeyboardInterrupt:
        print("Server interrupted by user")
    finally:
        server_socket.close()

if __name__ == "__main__": 
    main()

(★★★) You can pass an open file descriptor from one process to another via a socket. What syscall allows you to do this? When might this technique be useful?

sendmsg() / recvmsg() 