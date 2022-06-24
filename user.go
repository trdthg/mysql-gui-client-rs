package main

import "net"

type User struct {
	Name string
	Addr net.Addr
	C    chan string
	conn net.Conn
}

func NewUser(conn net.Conn) *User {
	return &User{
		Name: conn.RemoteAddr().String(),
		Addr: conn.RemoteAddr(),
		C:    make(chan string),
		conn: conn,
	}
}

func (user *User) ListenFromServer() {
	for {
		msg := <-user.C
		user.conn.Write([]byte(msg + "\n"))
	}
}
