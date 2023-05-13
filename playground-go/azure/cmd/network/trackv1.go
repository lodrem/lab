package main

import (
	"context"
	"os"

	"github.com/Azure/azure-sdk-for-go/services/compute/mgmt/2022-08-01/compute"
	"github.com/Azure/azure-sdk-for-go/services/network/mgmt/2022-07-01/network"
	"github.com/Azure/go-autorest/autorest/azure/auth"
)

type TrackV1 struct {
	vmssClient   compute.VirtualMachineScaleSetsClient
	vmssVMClient compute.VirtualMachineScaleSetVMsClient
	asgClient    network.ApplicationSecurityGroupsClient
}

func NewTrackV1(subscriptionID string) *TrackV1 {

	var (
		vmssClient   = compute.NewVirtualMachineScaleSetsClient(subscriptionID)
		vmssVMClient = compute.NewVirtualMachineScaleSetVMsClient(subscriptionID)
		asgClient    = network.NewApplicationSecurityGroupsClient(subscriptionID)
	)

	authorizer, err := auth.NewAuthorizerFromEnvironment()
	if err != nil {
		Logger.Error(err, "Failed to create authorizer from env")
		os.Exit(1)
	}

	vmssClient.Authorizer = authorizer
	vmssVMClient.Authorizer = authorizer
	asgClient.Authorizer = authorizer

	return &TrackV1{
		vmssClient,
		vmssVMClient,
		asgClient,
	}
}

func (v1 *TrackV1) MustCreateASG(ctx context.Context, location, resourceGroup, asgName string) string {
	resp, err := v1.asgClient.CreateOrUpdate(ctx, resourceGroup, asgName, network.ApplicationSecurityGroup{})
	if err != nil {
		Logger.Error(err, "Failed to begin create ASG", "asg-name", asgName)
		os.Exit(1)
	}
	if err := resp.WaitForCompletionRef(ctx, v1.asgClient.Client); err != nil {
		Logger.Error(err, "Failed to create ASG", "asg-name", asgName)
		os.Exit(1)
	}

	return resp.
}

func (v1 *TrackV1) MustUpdateVMSSWithASGs(ctx context.Context) {

}
