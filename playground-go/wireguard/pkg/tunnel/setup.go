package tunnel

import (
	"fmt"
	"log"
	"net/netip"

	"golang.zx2c4.com/wireguard/conn"
	"golang.zx2c4.com/wireguard/device"
	"golang.zx2c4.com/wireguard/ipc"
	"golang.zx2c4.com/wireguard/tun"
)

func Setup(interfaceName string, addr netip.Prefix) {
	tdev, err := tun.CreateTUN(interfaceName, device.DefaultMTU)
	if err != nil {
		log.Fatalf("Failed to create TUN device: %s", err)
	}

	fileUAPI, err := ipc.UAPIOpen(interfaceName)
	if err != nil {
		log.Fatalf("Failed to open UAPI socket: %s", err)
	}

	dev := device.NewDevice(tdev, conn.NewDefaultBind(), device.NewLogger(
		device.LogLevelVerbose,
		fmt.Sprintf("(%s) ", interfaceName),
	))
	defer dev.Close()
	log.Printf("Device started")

	uAPI, err := ipc.UAPIListen(interfaceName, fileUAPI)
	if err != nil {
		log.Fatalf("Failed to listen on UAPI socket: %s", err)
	}
	defer uAPI.Close()

	var (
		errC = make(chan error)
	)
	go func() {
		log.Printf("UAPI listener started")
		for {
			conn, err := uAPI.Accept()
			if err != nil {
				errC <- err
				return
			}
			go dev.IpcHandle(conn)
		}
	}()

	// Bind IP address to the interface
	LinkInterface(interfaceName, addr)

	select {
	case <-errC:
	case <-dev.Wait():
	}

	log.Printf("Shutting down")
}
