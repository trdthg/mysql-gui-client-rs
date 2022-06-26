package main

import (
	"fmt"
	"net"
	"strings"
)

type User struct {
	Name string
	Addr net.Addr
	C    chan string
	conn net.Conn
	s    *Server
}

func NewUser(conn net.Conn, s *Server) *User {
	return &User{
		Name: conn.RemoteAddr().String(),
		Addr: conn.RemoteAddr(),
		C:    make(chan string),
		conn: conn,
		s:    s,
	}
}

func (u *User) Online() {
	// 添加到在线中
	u.s.mapLock.Lock()
	u.s.OnlineMap[u.Name] = u
	u.s.mapLock.Unlock()

	// 广播消息
	u.s.BroadCastUser(u, "已上线")
}

func (u *User) Offline() {
	u.s.mapLock.Lock()
	delete(u.s.OnlineMap, u.Name)
	u.s.mapLock.Unlock()

	u.s.BroadCastUser(u, "已下线")
}

func (u *User) SendMsg(msg string) {
	u.conn.Write([]byte(msg))
}

func (u *User) HandleMessage(msg string) {
	if len(msg) == 0 {
		return
	}
	if msg[0] != '/' {
		u.s.BroadCastUser(u, msg)
		return
	}
	msg = msg[1:]
	if msg == "who" {
		u.s.mapLock.Lock()
		for _, user := range u.s.OnlineMap {
			fmt.Println("单点")
			msg := fmt.Sprintf("[%s] %s:在线...\n", user.Addr, user.Name)
			u.SendMsg(msg)
		}
		u.s.mapLock.Unlock()
	} else if len(msg) > 7 && msg[:7] == "rename " {
		name := msg[7:]
		u.s.mapLock.Lock()
		if _, ok := u.s.OnlineMap[name]; ok {
			u.SendMsg("用户名已被使用\n")
		} else {
			delete(u.s.OnlineMap, u.Name)
			u.Name = name
			u.s.OnlineMap[u.Name] = u
			u.SendMsg("更新成功 !\n")
		}
		u.s.mapLock.Unlock()
	} else if len(msg) > 3 && msg[:3] == "to " {
		msg = msg[3:]
		name := strings.Split(msg, ":")[0]
		if name == "" {
			u.SendMsg("消息格式不正确\n")
			return
		}
		user, ok := u.s.OnlineMap[name]
		if !ok {
			u.SendMsg("用户不存在或已下线\n")
			return
		}
		msg := strings.Split(msg, ":")[1]
		user.SendMsg(msg)
	}
}

func (user *User) ListenFromServer() {
	for {
		// log.Printf("[WARN] Loop3 Running...\n")
		msg, ok := <-user.C
		if !ok {
			return
		}
		user.conn.Write([]byte(msg + "\n"))
	}
}
