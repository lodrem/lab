package main

import (
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/spf13/cobra"
	"k8s.io/cli-runtime/pkg/printers"

	"github.com/lodrem/lab/golang/azure/pkg/echoserver"
	"github.com/lodrem/lab/golang/toolkit/ui"
)

func ManifestCmd() *cobra.Command {
	var (
		numReplicas int32
		numTCPPorts int
		numUDPPorts int
		outputFile  string
	)

	cmd := &cobra.Command{
		Use:   "manifest",
		Short: "Generate Kubernetes manifest for Bug Bash",
		Run: func(*cobra.Command, []string) {
			ui.Title("Generate Kubernetes manifest for feature AutoAssignHostPorts")

			p := printers.YAMLPrinter{}

			ports := make([]string, 0, numTCPPorts+numUDPPorts)
			tcpPorts := make([]int32, numTCPPorts)
			for i := 0; i < numTCPPorts; i++ {
				tcpPorts[i] = 40000 + int32(i)
				ports = append(ports, fmt.Sprintf("%d/tcp", tcpPorts[i]))
			}
			udpPorts := make([]int32, numUDPPorts)
			for i := 0; i < numUDPPorts; i++ {
				udpPorts[i] = 40000 + int32(i)
				ports = append(ports, fmt.Sprintf("%d/udp", udpPorts[i]))
			}

			annotations := map[string]string{
				"kubernetes.azure.com/assign-hostports-for-containerports": strings.Join(ports, ","),
			}

			es := echoserver.Options{
				NumReplicas:    numReplicas,
				TCPPorts:       tcpPorts,
				UDPPorts:       udpPorts,
				PodAnnotations: annotations,
			}

			var w io.Writer
			if outputFile != "" {
				f, err := os.Create(outputFile)
				if err != nil {
					ui.Error(err, "Failed to create output file")
				}
				defer f.Close()
				w = f
			} else {
				w = os.Stdout
			}

			err := p.PrintObj(es.Deployment(), w)
			if err != nil {
				ui.Error(err, "Failed to print deployment")
			}
		},
	}
	f := cmd.Flags()
	f.Int32Var(&numReplicas, "num-replicas", 3, "Number of replicas")
	f.IntVar(&numTCPPorts, "num-tcp-ports", 3, "Number of TCP ports")
	f.IntVar(&numUDPPorts, "num-udp-ports", 3, "Number of UDP ports")
	f.StringVar(&outputFile, "output-file", "", "Output file")

	return cmd
}
