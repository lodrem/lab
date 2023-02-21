package main

import (
	"context"
	"flag"
	"os"

	"github.com/Azure/azure-sdk-for-go/sdk/azidentity"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/compute/armcompute"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/network/armnetwork"
	"github.com/lodrem/lab/playground-go/toolkit/logging"
)

var Logger = logging.New()

var (
	SubscriptionID string
	ResourceGroup  string

	NSGName string
	ASG1ID  string
	ASG2ID  string

	VMSSName string
)

var (
	credential    *azidentity.DefaultAzureCredential
	asgClient     *armnetwork.ApplicationSecurityGroupsClient
	nsgClient     *armnetwork.SecurityGroupsClient
	nsgRuleClient *armnetwork.SecurityRulesClient
	vmssClient    *armcompute.VirtualMachineScaleSetsClient
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
}

func main() {
	var function string

	flag.StringVar(&SubscriptionID, "subscription-id", "", "Subscription ID")
	flag.StringVar(&ResourceGroup, "resource-group", "", "Resource Group")
	flag.StringVar(&function, "function", "", "Function to run")
	flag.StringVar(&NSGName, "nsg-name", "", "NSG Name")
	flag.StringVar(&ASG1ID, "asg-1-id", "", "ASG 1 ID")
	flag.StringVar(&ASG2ID, "asg-2-id", "", "ASG 2 ID")
	flag.Parse()

	ctx := context.Background()
	var err error
	switch function {
	case "create-security-rule-for-asg":
		err = CreateSecurityRuleForASG(ctx)
	}
	if err != nil {
		Logger.Error(err, "Failed to run function")
		os.Exit(1)
	}
}
