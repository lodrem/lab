package main

import (
	"context"
	"fmt"
	"math"
	"os"

	"github.com/Azure/azure-sdk-for-go/sdk/azcore/to"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/compute/armcompute"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/network/armnetwork"
)

func MustCreateASG(ctx context.Context, location, resourceGroup, asgName string) string {
	Logger.Info("Creating ASG", "asg-name", asgName)
	poller, err := asgClient.BeginCreateOrUpdate(ctx, resourceGroup, asgName, armnetwork.ApplicationSecurityGroup{
		Location:   to.Ptr(location),
		Properties: &armnetwork.ApplicationSecurityGroupPropertiesFormat{},
	}, nil)
	if err != nil {
		Logger.Error(err, "Failed to begin create ASG", "asg-name", asgName)
		os.Exit(1)
	}

	resp, err := poller.PollUntilDone(ctx, nil)
	if err != nil {
		Logger.Error(err, "Failed to create ASG", "asg-name", asgName)
		os.Exit(1)
	}

	return *resp.ID
}

func MustDeleteASG(ctx context.Context, resourceGroup, asgName string) {
	Logger.Info("Deleting ASG", "asg-name", asgName)
	poller, err := asgClient.BeginDelete(ctx, resourceGroup, asgName, nil)
	if err != nil {
		Logger.Error(err, "Failed to begin delete ASG", "asg-name", asgName)
		os.Exit(1)
	}
	if _, err := poller.PollUntilDone(ctx, nil); err != nil {
		Logger.Error(err, "Failed to delete ASG", "asg-name", asgName)
		os.Exit(1)
	}
}

func MustGetVMSS(ctx context.Context, resourceGroup, vmssName string) armcompute.VirtualMachineScaleSet {
	resp, err := vmssClient.Get(ctx, resourceGroup, vmssName, nil)
	if err != nil {
		Logger.Error(err, "Failed to get VMSS", "vmss-name", vmssName)
		os.Exit(1)
	}

	return resp.VirtualMachineScaleSet
}

func MustUpdateVMSSWithASGs(ctx context.Context, resourceGroup, vmssName string, asgIDs []string) {
	Logger.Info("Updating VMSS", "vmss-name", vmssName)
	asgs := make([]*armcompute.SubResource, 0, len(asgIDs))
	for _, asgID := range asgIDs {
		asgs = append(asgs, &armcompute.SubResource{
			ID: to.Ptr(asgID),
		})
	}

	vmss := MustGetVMSS(ctx, resourceGroup, vmssName)
	vmss.Properties.VirtualMachineProfile.NetworkProfile.NetworkInterfaceConfigurations[0].Properties.IPConfigurations[0].Properties.ApplicationSecurityGroups = asgs
	poller, err := vmssClient.BeginCreateOrUpdate(ctx, resourceGroup, vmssName, vmss, nil)
	if err != nil {
		Logger.Error(err, "Failed to begin update VMSS", "vmss-name", vmssName, "asg-ids", asgIDs)
		os.Exit(1)
	}

	if _, err := poller.PollUntilDone(ctx, nil); err != nil {
		Logger.Error(err, "Failed to update VMSS", "vmss-name", vmssName, "asg-ids", asgIDs)
		os.Exit(1)
	}

	for i := 0; i < 3; i++ {
		instanceID := fmt.Sprintf("%d", i)
		MustUpdateVMSSInstanceWithASGs(ctx, resourceGroup, vmssName, instanceID, asgIDs)
	}
}

func MustGetVMSSInstance(ctx context.Context, resourceGroup, vmssName, instanceID string) armcompute.VirtualMachineScaleSetVM {
	resp, err := vmssVMClient.Get(ctx, resourceGroup, vmssName, instanceID, nil)
	if err != nil {
		Logger.Error(err, "Failed to get VMSS Instance", "vmss-name", vmssName, "instance-id", instanceID)
		os.Exit(1)
	}

	return resp.VirtualMachineScaleSetVM
}

func MustUpdateVMSSInstanceWithASGs(ctx context.Context, resourceGroup, vmssName, instanceID string, asgIDs []string) {
	Logger.Info("Updating VMSS Instance", "vmss-name", vmssName, "instance-id", instanceID)
	asgs := make([]*armcompute.SubResource, 0, len(asgIDs))
	for _, asgID := range asgIDs {
		asgs = append(asgs, &armcompute.SubResource{
			ID: to.Ptr(asgID),
		})
	}

	vm := MustGetVMSSInstance(ctx, resourceGroup, vmssName, instanceID)
	vm.Properties.NetworkProfileConfiguration.NetworkInterfaceConfigurations[0].Properties.IPConfigurations[0].Properties.ApplicationSecurityGroups = asgs

	poller, err := vmssVMClient.BeginUpdate(ctx, resourceGroup, vmssName, instanceID, vm, nil)
	if err != nil {
		Logger.Error(err, "Failed to begin update VMSS Instance", "vmss-name", vmssName, "instance-id", instanceID, "asg-ids", asgIDs)
		os.Exit(1)
	}

	if _, err := poller.PollUntilDone(ctx, nil); err != nil {
		Logger.Error(err, "Failed to update VMSS Instance", "vmss-name", vmssName, "instance-id", instanceID, "asg-ids", asgIDs)
		os.Exit(1)
	}
}

func Test() {
	const (
		Location      = "eastus"
		ResourceGroup = "jialuncai"
		ASG1Name      = "asg-1"
		ASG2Name      = "asg-2"
		VMSSName      = "testing-vmss"
	)

	var (
		ctx = context.Background()
	)

	Logger.Info("Starting...")

	asgID1 := MustCreateASG(ctx, Location, ResourceGroup, ASG1Name)
	asgID2 := MustCreateASG(ctx, Location, ResourceGroup, ASG2Name)

	for i := 0; i < math.MaxUint32; i++ {
		targetASGName := fmt.Sprintf("target-asg-%d", i)
		targetASGID := MustCreateASG(ctx, Location, ResourceGroup, targetASGName)

		MustUpdateVMSSWithASGs(ctx, ResourceGroup, VMSSName, []string{targetASGID})
		MustUpdateVMSSWithASGs(ctx, ResourceGroup, VMSSName, []string{asgID1, asgID2})
		MustDeleteASG(ctx, ResourceGroup, targetASGName)
	}
}
