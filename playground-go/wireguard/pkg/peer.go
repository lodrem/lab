package pkg

import (
	"fmt"
	"log"
	"net"
	"net/netip"

	"golang.zx2c4.com/wireguard/wgctrl/wgtypes"
)

type Peer struct {
	PrivateKey *wgtypes.Key
	Address    netip.Prefix
	Endpoint   *net.UDPAddr
}

func New(key, addr string, endpoint *net.UDPAddr) (*Peer, error) {
	k, err := wgtypes.ParseKey(key)
	if err != nil {
		return nil, fmt.Errorf("parse private key: %w", err)
	}
	a, err := netip.ParsePrefix(addr)
	if err != nil {
		return nil, fmt.Errorf("parse address: %w", err)
	}

	return &Peer{
		PrivateKey: &k,
		Address:    a,
		Endpoint:   endpoint,
	}, nil
}

func (peer *Peer) Config(allowedIPs []net.IPNet) wgtypes.PeerConfig {
	return wgtypes.PeerConfig{
		PublicKey:  peer.PrivateKey.PublicKey(),
		Endpoint:   peer.Endpoint,
		AllowedIPs: allowedIPs,
	}
}

var (
	Server *Peer
	Client *Peer
)

func init() {
	{
		server, err := New(
			"kCTdS3qJ36wK5iRmsgb6YgMWq7213FlN7PeYLMOCN0c=",
			"10.20.0.1/32",
			&net.UDPAddr{IP: net.IPv4(0, 0, 0, 0), Port: 56324},
		)
		if err != nil {
			log.Fatalf("Failed to create server peer: %s", err)
		}
		Server = server
	}
	{
		client, err := New(
			"GIT7VTyioNRp0J/CW0tnFkyZw4UqLY1Cnx7xRuWTEEw=",
			"10.20.0.2/32",
			nil,
		)
		if err != nil {
			log.Fatalf("Failed to create client peer: %s", err)
		}
		Client = client
	}
}
