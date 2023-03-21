package main

import (
	"log"
	"net"
	"time"

	"golang.zx2c4.com/wireguard/wgctrl"
	"golang.zx2c4.com/wireguard/wgctrl/wgtypes"

	"github.com/lodrem/lab/playground-go/wireguard/pkg"
	"github.com/lodrem/lab/playground-go/wireguard/pkg/tunnel"
	"github.com/lodrem/lab/playground-go/wireguard/pkg/ui"
)

func main() {
	const iface = "utun42"

	ctrl, err := wgctrl.New()
	if err != nil {
		panic(err)
	}

	go tunnel.Setup(iface, pkg.Client.Address)

	time.Sleep(5 * time.Second) // Wait for interface to be created.

	ui.ListDevices(ctrl)
	if err := ctrl.ConfigureDevice(iface, wgtypes.Config{
		PrivateKey: pkg.Client.PrivateKey,
		Peers: []wgtypes.PeerConfig{
			pkg.Server.Config([]net.IPNet{
				{
					IP:   net.IPv4(10, 20, 0, 1),
					Mask: net.IPv4Mask(255, 255, 255, 255),
				},
			}),
		},
	}); err != nil {
		log.Fatalf("Failed to configure device: %s", err)
	}

	time.Sleep(1 * time.Hour)
}
