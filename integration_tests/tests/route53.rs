#![cfg(feature = "route53")]

extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_route53::{Route53, Route53Client, ListHostedZonesRequest, ChangeResourceRecordSetsRequest, ChangeBatch, Change, ResourceRecordSet};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_hosted_zones() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        Route53Client::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHostedZonesRequest::default();

    client.list_hosted_zones(&request).unwrap();
}

#[test]
fn should_create_a_record() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        Route53Client::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let resource_record_set = ResourceRecordSet {
        name: "my full qualified domain".to_owned(),
        type_: "A".to_owned(),
        ..Default::default()
    };
    let change = Change {
        action: "CREATE".to_owned(),
        resource_record_set: resource_record_set,
    };
    let change_batch = ChangeBatch {
        changes: vec!(change),
        ..Default::default()
    };
    let request = ChangeResourceRecordSetsRequest {
        change_batch: change_batch,
        hosted_zone_id: "my-hosted-zone".to_owned(),
    };

    client.change_resource_record_sets(&request).expect("Couldn't create resource record set");
}