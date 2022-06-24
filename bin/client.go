package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
)

func main() {
	//
	conn, err := net.Dial("tcp", "localhost:1234")
	if err != nil {
		log.Printf("Failed to connect to server: %s", err.Error())
	}

	for {
		msg, err := bufio.NewReader(conn).ReadString('\n')
		if err != nil {
			fmt.Printf("WARN: %s", err.Error())
			break
		}
		fmt.Printf("%s", msg)
	}
}
