package main

import (
	"os"

	"github.com/Azure/azure-sdk-for-go/sdk/azidentity"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/compute/armcompute"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/network/armnetwork"

	"github.com/lodrem/lab/playground-go/toolkit/logging"
)

var Logger = logging.New()

var (
	SubscriptionID string = "8ecadfc9-d1a3-4ea4-b844-0d9f87e4d7c8"
)

var (
	credential    *azidentity.DefaultAzureCredential
	asgClient     *armnetwork.ApplicationSecurityGroupsClient
	nsgClient     *armnetwork.SecurityGroupsClient
	nsgRuleClient *armnetwork.SecurityRulesClient
	vmssClient    *armcompute.VirtualMachineScaleSetsClient
	vmssVMClient  *armcompute.VirtualMachineScaleSetVMsClient
)

func init() {
	var err error
	credential, err = azidentity.NewDefaultAzureCredential(nil)
	if err != nil {
		Logger.Error(err, "Failed to create credential")
		os.Exit(1)
	}

	asgClient, err = armnetwork.NewApplicationSecurityGroupsClient(SubscriptionID, credential, nil)
	if err != nil {
		Logger.Error(err, "Failed to create application security groups client")
		os.Exit(1)
	}

	nsgClient, err = armnetwork.NewSecurityGroupsClient(SubscriptionID, credential, nil)
	if err != nil {
		Logger.Error(err, "Failed to create security groups client")
		os.Exit(1)
	}

	nsgRuleClient, err = armnetwork.NewSecurityRulesClient(SubscriptionID, credential, nil)
	if err != nil {
		Logger.Error(err, "Failed to create security rules client")
		os.Exit(1)
	}

	vmssClient, err = armcompute.NewVirtualMachineScaleSetsClient(SubscriptionID, credential, nil)
	if err != nil {
		Logger.Error(err, "Failed to create virtual machine scale sets client")
		os.Exit(1)
	}
	vmssVMClient, err = armcompute.NewVirtualMachineScaleSetVMsClient(SubscriptionID, credential, nil)
	if err != nil {
		Logger.Error(err, "Failed to create virtual machine scale set vms client")
		os.Exit(1)
	}
}

func main() {
	Test()
}
