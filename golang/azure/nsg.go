package main

import (
	"context"

	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/network/armnetwork"
	"github.com/Azure/go-autorest/autorest/to"
)

func CreateSecurityRuleForASG(ctx context.Context) error {
	nsgResp, err := nsgClient.Get(ctx, ResourceGroup, NSGName, nil)
	if err != nil {
		Logger.Error(err, "Failed to get NSG")
		return err
	}
	nsg := nsgResp.SecurityGroup

	var (
		protocol  = armnetwork.SecurityRuleProtocolTCP
		access    = armnetwork.SecurityRuleAccessAllow
		direction = armnetwork.SecurityRuleDirectionInbound
	)

	rule := armnetwork.SecurityRule{
		Name: to.StringPtr("SetupASG"),
		Properties: &armnetwork.SecurityRulePropertiesFormat{
			Description:     to.StringPtr("Setup ASG"),
			Protocol:        &protocol,
			SourcePortRange: to.StringPtr("*"),
			DestinationPortRanges: []*string{
				to.StringPtr("80"),
				to.StringPtr("443"),
				to.StringPtr("8080"),
				to.StringPtr("8081-8081"),
				to.StringPtr("33333-33333"),
				to.StringPtr("40000-50000"),
			},
			SourceAddressPrefix: to.StringPtr("*"),
			DestinationApplicationSecurityGroups: []*armnetwork.ApplicationSecurityGroup{
				{
					ID: to.StringPtr(ASG1ID),
				},
				{
					ID: to.StringPtr(ASG2ID),
				},
			},
			Access:    &access,
			Direction: &direction,
			Priority:  to.Int32Ptr(657),
		},
	}
	nsg.Properties.SecurityRules = append(nsg.Properties.SecurityRules, &rule)
	poll, err := nsgClient.BeginCreateOrUpdate(ctx, ResourceGroup, NSGName, nsg, nil)
	if err != nil {
		Logger.Error(err, "Failed to create security rule")
		return err
	}
	resp, err := poll.PollUntilDone(ctx, nil)
	if err != nil {
		Logger.Error(err, "Failed to create security rule")
		return err
	}
	_ = resp.ID
	return nil
}
