package main

import (
	"fmt"
	"io"
	"log"
	"net"
	"sync"
	"time"
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
func (s *Server) OnlineUser(u *User) {
	s.mapLock.Lock()
	s.OnlineMap[u.Name] = u
	s.mapLock.Unlock()
	s.BroadCastUser(u, "已上线")
}
func (s *Server) OfflineUser(u *User) {
	s.mapLock.Lock()
	delete(s.OnlineMap, u.Name)
	s.mapLock.Unlock()
	s.BroadCastUser(u, "已下线")
}

func (s *Server) Handle(conn net.Conn) {
	log.Printf("[WARN] Loop1 Running...\n")
	user := NewUser(conn, s)

	// s.OnlineUser(user)
	user.Online()
	isAlive := make(chan struct{})

	// 接收系统广播消息
	go user.ListenFromServer()

	// 接收用户的消息
	go func() {
		b := make([]byte, 4096)
		for {
			n, err := user.conn.Read(b)
			if n == 0 {
				user.Offline()
				// s.OfflineUser(user)
				return
			}
			if err != nil && err != io.EOF {
				fmt.Printf("[WARN] Read Failed: %s", err.Error())
				return
			}
			// n-1 去掉最后的 \n
			user.HandleMessage(string(b[:n-1]))
			isAlive <- struct{}{}
		}
	}()

	for {
		select {
		case <-isAlive:
			//
		case <-time.After(time.Second * 10):
			user.SendMsg("您已被踢出服务器\n")
			close(user.C)
			conn.Close()
			user.Offline()
			return
		}
	}
}

func (s *Server) BroadCast(msg string) {
	s.Messages <- msg
	fmt.Println(msg)
}

func (s *Server) BroadCastUser(user *User, msg string) {
	msg = fmt.Sprintf("[%s]: %s", user.Name, msg)
	fmt.Println(msg)
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
	fmt.Printf("[INFO] 服务端启动 %s\n", addr)
	defer l.Close()

	// 分发消息
	go s.HandleMessages()

	// 接收连接
	for {
		conn, err := l.Accept()
		if err != nil {
			log.Printf("[WARN] Accept Failed: %s\n", err.Error())
			continue
		}
		go s.Handle(conn)
	}
}
