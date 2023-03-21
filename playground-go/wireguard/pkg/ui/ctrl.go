package ui

import (
	"log"

	"golang.zx2c4.com/wireguard/wgctrl"
)

func ListDevices(ctrl *wgctrl.Client) {
	devices, err := ctrl.Devices()
	if err != nil {
		log.Fatalf("Failed to list devices: %s", err)
	}

	log.Printf("Devices[%d]:", len(devices))
	for _, device := range devices {
		log.Printf("  %s - %s", device.Name, device.Type)
	}
}
