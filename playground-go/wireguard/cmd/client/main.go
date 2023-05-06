package main

import (
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"golang.org/x/crypto/chacha20poly1305"
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

	var key = []byte("0123456789abcdef0123456789abcdef")
	cipher, _ := chacha20poly1305.New(key)
	var dst []byte
	var nonce [chacha20poly1305.NonceSize]byte
	var aad = []byte("fedcba9876543210")
	var data = []byte("foobar")
	binary.LittleEndian.PutUint64(nonce[4:], uint64(42))

	fmt.Printf("aead = %s\n", hex.EncodeToString(cipher.Seal(dst, nonce[:], data, aad)))

	cipher, _ = chacha20poly1305.NewX(key)
	var xnonce = [chacha20poly1305.NonceSizeX]byte([]byte("0123456789abcdef01234567"))
	fmt.Printf("xaead = %s\n", hex.EncodeToString(cipher.Seal(dst, xnonce[:], data, aad)))

	return

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
