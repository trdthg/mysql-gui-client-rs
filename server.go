package main

import (
	"fmt"
	"log"
	"net"
	"sync"
)

type Server struct {
	Ip        string
	Port      int
	mapLock   sync.RWMutex
	OnlineMap map[string]*User
	Messages  chan string
}

func NewServer(ip string, port int) *Server {
	return &Server{
		Ip:        ip,
		Port:      port,
		Messages:  make(chan string),
		mapLock:   sync.RWMutex{},
		OnlineMap: map[string]*User{},
	}
}

func (s *Server) Handle(conn net.Conn) {
	defer conn.Close()
	user := NewUser(conn)

	// 添加到在线中
	s.mapLock.Lock()
	s.OnlineMap[user.Name] = user
	s.mapLock.Unlock()

	// 广播消息
	s.BroadCast(fmt.Sprintf("用户 %s 连接至服务器\n", conn.RemoteAddr()))

	// 接收别人的新消息
	go user.ListenFromServer()

	// 接收用户的消息
	for {
		b := []byte{}
		n, err := user.conn.Read(b)
		if err != nil || n == 0 {
			continue
		}
		user.C <- string(b[:n])
	}
	// delete(s.OnlineMap, user.Name)
}

func (s *Server) BroadCast(msg string) {
	s.Messages <- msg

}

func (s *Server) HandleMessages() {
	for {
		msg := <-s.Messages
		s.mapLock.Lock()
		for _, user := range s.OnlineMap {
			user.C <- msg
		}
		s.mapLock.Unlock()
	}
}

func (s *Server) Run() {
	// 绑定端口
	addr := fmt.Sprintf("%s:%d", s.Ip, s.Port)
	l, err := net.Listen("tcp", addr)
	if err != nil {
		log.Printf("Listen Failed: %s", err.Error())
	}
	fmt.Printf("服务端启动 %s\n", addr)
	defer l.Close()

	// 分发消息
	go s.HandleMessages()

	// 接收连接
	for {
		fmt.Printf("开始处理请求\n")
		conn, err := l.Accept()
		if err != nil {
			log.Printf("Accept Failed: %s\n", err.Error())
			continue
		}
		go s.Handle(conn)
	}
}
