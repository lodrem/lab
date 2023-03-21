package tunnel

import (
	"log"
	"net/netip"
	"os/exec"
)

func LinkInterface(name string, addr netip.Prefix) {
	args := []string{name, inet(addr), addr.String(), addr.Addr().String()}
	out, err := exec.Command("ifconfig", args...).CombinedOutput()
	if err != nil {
		log.Fatalf("Failed to configure interface: %s: %s", err, out)
	}
	log.Printf("Interface configured: %s", out)
}

func inet(p netip.Prefix) string {
	if p.Addr().Is6() {
		return "inet6"
	}
	return "inet"
}
