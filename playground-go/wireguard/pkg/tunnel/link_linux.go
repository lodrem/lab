package tunnel

import (
	"log"
	"net/netip"

	"github.com/vishvananda/netlink"
)

func LinkInterface(name string, addr netip.Prefix) {
	iface, err := netlink.LinkByName(name)
	if err != nil {
		log.Fatalf("Failed to get interface: %s", err)
	}
	a, _ := netlink.ParseAddr(addr.String())
	if err := netlink.AddrAdd(iface, a); err != nil {
		log.Fatalf("Failed to add address: %s", err)
	}
}
