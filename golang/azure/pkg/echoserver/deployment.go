package echoserver

import (
	"fmt"
	"strings"

	appsv1 "k8s.io/api/apps/v1"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/resource"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
)

func portsToString(ports []int32) string {
	var b strings.Builder
	for i, port := range ports {
		if i > 0 {
			b.WriteString(",")
		}
		b.WriteString(fmt.Sprintf("%d", port))
	}
	return b.String()
}

type Options struct {
	Name           string
	Image          string
	NumReplicas    int32
	PodAnnotations map[string]string
	TCPPorts       []int32
	UDPPorts       []int32
	HTTPPorts      []int32
}

func (opts *Options) defaults() {
	if opts.Name == "" {
		opts.Name = "echoserver"
	}
	if opts.Image == "" {
		opts.Image = "ghcr.io/lodrem/echoserver:v0.0.4"
	}
	if opts.NumReplicas == 0 {
		opts.NumReplicas = 3
	}
}

func (opts *Options) Deployment() *appsv1.Deployment {
	opts.defaults()

	command := []string{
		"/usr/local/bin/echoserver",
	}
	containerPorts := make([]corev1.ContainerPort, 0, len(opts.TCPPorts)+len(opts.UDPPorts)+len(opts.HTTPPorts))

	if len(opts.TCPPorts) > 0 {
		command = append(command,
			"-enable-tcp=true",
			fmt.Sprintf("--tcp-ports=%s", portsToString(opts.TCPPorts)),
		)
		for _, port := range opts.TCPPorts {
			containerPorts = append(containerPorts, corev1.ContainerPort{
				Name:          fmt.Sprintf("tcp-%d", port),
				ContainerPort: port,
				Protocol:      corev1.ProtocolTCP,
			})
		}
	}
	if len(opts.UDPPorts) > 0 {
		command = append(command,
			"-enable-udp=true",
			fmt.Sprintf("--udp-ports=%s", portsToString(opts.UDPPorts)),
		)
		for _, port := range opts.UDPPorts {
			containerPorts = append(containerPorts, corev1.ContainerPort{
				Name:          fmt.Sprintf("udp-%d", port),
				ContainerPort: port,
				Protocol:      corev1.ProtocolUDP,
			})
		}
	}
	if len(opts.HTTPPorts) > 0 {
		command = append(command,
			"-enable-http=true",
			fmt.Sprintf("--http-ports=%s", portsToString(opts.HTTPPorts)),
		)
		for _, port := range opts.HTTPPorts {
			containerPorts = append(containerPorts, corev1.ContainerPort{
				Name:          fmt.Sprintf("http-%d", port),
				ContainerPort: port,
				Protocol:      corev1.ProtocolTCP,
			})
		}
	}

	d := &appsv1.Deployment{
		ObjectMeta: metav1.ObjectMeta{
			Name: fmt.Sprintf("%s-deployment", opts.Name),
			Labels: map[string]string{
				"app": opts.Name,
			},
		},
		Spec: appsv1.DeploymentSpec{
			Replicas: &opts.NumReplicas,
			Selector: &metav1.LabelSelector{
				MatchLabels: map[string]string{
					"app": opts.Name,
				},
			},
			Template: corev1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{
					Name: opts.Name,
					Labels: map[string]string{
						"app": opts.Name,
					},
					Annotations: opts.PodAnnotations,
				},
				Spec: corev1.PodSpec{
					Containers: []corev1.Container{
						{
							Name:    opts.Name,
							Image:   opts.Image,
							Command: command,
							Resources: corev1.ResourceRequirements{
								Requests: corev1.ResourceList{
									corev1.ResourceCPU:    resource.MustParse("100m"),
									corev1.ResourceMemory: resource.MustParse("128Mi"),
								},
							},
							Ports: containerPorts,
						},
					},
				},
			},
		},
	}
	d.GetObjectKind().SetGroupVersionKind(appsv1.SchemeGroupVersion.WithKind("Deployment"))

	return d
}
