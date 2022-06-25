package main

import (
	"bufio"
	"errors"
	"flag"
	"fmt"
	"log"
	"net"
	"os"
)

var serverIp string
var serverPort int

func init() {
	flag.StringVar(&serverIp, "ip", "localhsot", "输入服务端 IP 地址")
	flag.IntVar(&serverPort, "port", 1234, "输入服务端端口号")
}

type Client struct {
	Port int
	Ip   string
	conn net.Conn
}

func NewClient() (*Client, error) {
	conn, err := net.Dial("tcp", "localhost:1234")
	if err != nil {
		return nil, errors.New(fmt.Sprintf("Failed to connect to server: %s", err.Error()))
	}
	return &Client{
		Ip:   serverIp,
		Port: serverPort,
		conn: conn,
	}, nil
}

func (c *Client) ReadInput() {

}


func main() {
	flag.Parse()
	client, err := NewClient()
	if err != nil {
		log.Fatal(fmt.Sprintf("%s", err.Error()))
		return
	}

	select {}
	inputs := make(chan string, 3)
	go func() {
		sc := bufio.NewScanner(os.Stdin)
		for {
			sc.Scan()
			inputs <- sc.Text()
		}
	}()
	msgs := make(chan string, 100)
	go func() {
		msg, err := bufio.NewReader(conn).ReadString('\n')
		if err != nil {
			fmt.Printf("WARN: %s", err.Error())
			return
		}
		msgs <- msg
	}()
	for {
		select {
		case input := <-inputs:
			conn.Write([]byte(input))
		case msg := <-msgs:
			fmt.Printf("%s", msg)
		}

	}
}
