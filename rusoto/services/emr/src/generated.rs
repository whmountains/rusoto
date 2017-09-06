
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use std::default::Default;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddInstanceFleetInput {
    #[doc="<p>The unique identifier of the cluster.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>Specifies the configuration of the instance fleet.</p>"]
    #[serde(rename="InstanceFleet")]
    pub instance_fleet: InstanceFleetConfig,
}
impl AddInstanceFleetInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddInstanceFleetInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `instance_fleet`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddInstanceFleetInput.instance_fleet = value.into();`.
    pub fn instance_fleet<ValueType: Into<InstanceFleetConfig>>(mut self,
                                                                value: ValueType)
                                                                -> Self {
        self.instance_fleet = value.into();
        self
    }
    /// Returns a new instance of AddInstanceFleetInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>, InstanceFleetType: Into<InstanceFleetConfig>>
        (cluster_id: ClusterIdType,
         instance_fleet: InstanceFleetType)
         -> AddInstanceFleetInput {
        AddInstanceFleetInput {
            cluster_id: cluster_id.into(),
            instance_fleet: instance_fleet.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddInstanceFleetOutput {
    #[doc="<p>The unique identifier of the cluster.</p>"]
    #[serde(rename="ClusterId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc="<p>The unique identifier of the instance fleet.</p>"]
    #[serde(rename="InstanceFleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleet_id: Option<String>,
}
#[doc="<p>Input to an AddInstanceGroups call.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddInstanceGroupsInput {
    #[doc="<p>Instance groups to add.</p>"]
    #[serde(rename="InstanceGroups")]
    pub instance_groups: Vec<InstanceGroupConfig>,
    #[doc="<p>Job flow in which to add the instance groups.</p>"]
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
}
impl AddInstanceGroupsInput {
    /// Sets `instance_groups`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddInstanceGroupsInput.instance_groups = value.into();`.
    pub fn instance_groups<ValueType: Into<Vec<InstanceGroupConfig>>>(mut self,
                                                                      value: ValueType)
                                                                      -> Self {
        self.instance_groups = value.into();
        self
    }
    /// Sets `job_flow_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddInstanceGroupsInput.job_flow_id = value.into();`.
    pub fn job_flow_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.job_flow_id = value.into();
        self
    }
    /// Returns a new instance of AddInstanceGroupsInput with optional fields set to `None`.
    pub fn new<InstanceGroupsType: Into<Vec<InstanceGroupConfig>>, JobFlowIdType: Into<String>>
        (instance_groups: InstanceGroupsType,
         job_flow_id: JobFlowIdType)
         -> AddInstanceGroupsInput {
        AddInstanceGroupsInput {
            instance_groups: instance_groups.into(),
            job_flow_id: job_flow_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Output from an AddInstanceGroups call.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddInstanceGroupsOutput {
    #[doc="<p>Instance group IDs of the newly created instance groups.</p>"]
    #[serde(rename="InstanceGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_ids: Option<Vec<String>>,
    #[doc="<p>The job flow ID in which the instance groups are added.</p>"]
    #[serde(rename="JobFlowId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flow_id: Option<String>,
}
#[doc="<p> The input argument to the <a>AddJobFlowSteps</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddJobFlowStepsInput {
    #[doc="<p>A string that uniquely identifies the job flow. This identifier is returned by <a>RunJobFlow</a> and can also be obtained from <a>ListClusters</a>. </p>"]
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
    #[doc="<p> A list of <a>StepConfig</a> to be executed by the job flow. </p>"]
    #[serde(rename="Steps")]
    pub steps: Vec<StepConfig>,
}
impl AddJobFlowStepsInput {
    /// Sets `job_flow_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddJobFlowStepsInput.job_flow_id = value.into();`.
    pub fn job_flow_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.job_flow_id = value.into();
        self
    }
    /// Sets `steps`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddJobFlowStepsInput.steps = value.into();`.
    pub fn steps<ValueType: Into<Vec<StepConfig>>>(mut self, value: ValueType) -> Self {
        self.steps = value.into();
        self
    }
    /// Returns a new instance of AddJobFlowStepsInput with optional fields set to `None`.
    pub fn new<JobFlowIdType: Into<String>, StepsType: Into<Vec<StepConfig>>>
        (job_flow_id: JobFlowIdType,
         steps: StepsType)
         -> AddJobFlowStepsInput {
        AddJobFlowStepsInput {
            job_flow_id: job_flow_id.into(),
            steps: steps.into(),
            ..Default::default()
        }
    }
}
#[doc="<p> The output for the <a>AddJobFlowSteps</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddJobFlowStepsOutput {
    #[doc="<p>The identifiers of the list of steps added to the job flow.</p>"]
    #[serde(rename="StepIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_ids: Option<Vec<String>>,
}
#[doc="<p>This input identifies a cluster and a list of tags to attach.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddTagsInput {
    #[doc="<p>The Amazon EMR resource identifier to which tags will be added. This value must be a cluster identifier.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>A list of tags to associate with a cluster and propagate to EC2 instances. Tags are user-defined key/value pairs that consist of a required key string with a maximum of 128 characters, and an optional value string with a maximum of 256 characters.</p>"]
    #[serde(rename="Tags")]
    pub tags: Vec<Tag>,
}
impl AddTagsInput {
    /// Sets `resource_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddTagsInput.resource_id = value.into();`.
    pub fn resource_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.resource_id = value.into();
        self
    }
    /// Sets `tags`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AddTagsInput.tags = value.into();`.
    pub fn tags<ValueType: Into<Vec<Tag>>>(mut self, value: ValueType) -> Self {
        self.tags = value.into();
        self
    }
    /// Returns a new instance of AddTagsInput with optional fields set to `None`.
pub fn new<ResourceIdType: Into<String>, TagsType: Into<Vec<Tag>>>(resource_id: ResourceIdType, tags: TagsType) -> AddTagsInput{
        AddTagsInput {
            resource_id: resource_id.into(),
            tags: tags.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output indicates the result of adding tags to a resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddTagsOutput;

#[doc="<p>An application is any Amazon or third-party software that you can add to the cluster. This structure contains a list of strings that indicates the software to use with the cluster and accepts a user argument list. Amazon EMR accepts and forwards the argument list to the corresponding installation script as bootstrap action argument. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/ManagementGuide/emr-mapr.html\">Using the MapR Distribution for Hadoop</a>. Currently supported values are:</p> <ul> <li> <p>\"mapr-m3\" - launch the cluster using MapR M3 Edition.</p> </li> <li> <p>\"mapr-m5\" - launch the cluster using MapR M5 Edition.</p> </li> <li> <p>\"mapr\" with the user arguments specifying \"--edition,m3\" or \"--edition,m5\" - launch the cluster using MapR M3 or M5 Edition, respectively.</p> </li> </ul> <note> <p>In Amazon EMR releases 4.x and later, the only accepted parameter is the application name. To pass arguments to applications, you supply a configuration for each application.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Application {
    #[doc="<p>This option is for advanced users only. This is meta information about third-party applications that third-party vendors use for testing purposes.</p>"]
    #[serde(rename="AdditionalInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_info: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Arguments for Amazon EMR to pass to the application.</p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<Vec<String>>,
    #[doc="<p>The name of the application.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The version of the application.</p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,
}
impl Application {
    /// Sets `additional_info`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Application.additional_info = Some(value.into());`.
    pub fn additional_info<ValueType: Into<::std::collections::HashMap<String, String>>>
        (mut self,
         value: ValueType)
         -> Self {
        self.additional_info = Some(value.into());
        self
    }
    /// Sets `args`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Application.args = Some(value.into());`.
    pub fn args<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.args = Some(value.into());
        self
    }
    /// Sets `name`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Application.name = Some(value.into());`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets `version`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Application.version = Some(value.into());`.
    pub fn version<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.version = Some(value.into());
        self
    }
    /// Returns a new instance of Application with optional fields set to `None`.
    pub fn new() -> Application {
        Application { ..Default::default() }
    }
}
#[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. An automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AutoScalingPolicy {
    #[doc="<p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activity will not cause an instance group to grow above or below these limits.</p>"]
    #[serde(rename="Constraints")]
    pub constraints: ScalingConstraints,
    #[doc="<p>The scale-in and scale-out rules that comprise the automatic scaling policy.</p>"]
    #[serde(rename="Rules")]
    pub rules: Vec<ScalingRule>,
}
impl AutoScalingPolicy {
    /// Sets `constraints`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AutoScalingPolicy.constraints = value.into();`.
    pub fn constraints<ValueType: Into<ScalingConstraints>>(mut self, value: ValueType) -> Self {
        self.constraints = value.into();
        self
    }
    /// Sets `rules`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `AutoScalingPolicy.rules = value.into();`.
    pub fn rules<ValueType: Into<Vec<ScalingRule>>>(mut self, value: ValueType) -> Self {
        self.rules = value.into();
        self
    }
    /// Returns a new instance of AutoScalingPolicy with optional fields set to `None`.
    pub fn new<ConstraintsType: Into<ScalingConstraints>, RulesType: Into<Vec<ScalingRule>>>
        (constraints: ConstraintsType,
         rules: RulesType)
         -> AutoScalingPolicy {
        AutoScalingPolicy {
            constraints: constraints.into(),
            rules: rules.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AutoScalingPolicyDescription {
    #[doc="<p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activity will not cause an instance group to grow above or below these limits.</p>"]
    #[serde(rename="Constraints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub constraints: Option<ScalingConstraints>,
    #[doc="<p>The scale-in and scale-out rules that comprise the automatic scaling policy.</p>"]
    #[serde(rename="Rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<Vec<ScalingRule>>,
    #[doc="<p>The status of an automatic scaling policy. </p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<AutoScalingPolicyStatus>,
}
#[doc="<p>The reason for an <a>AutoScalingPolicyStatus</a> change.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AutoScalingPolicyStateChangeReason {
    #[doc="<p>The code indicating the reason for the change in status.<code>USER_REQUEST</code> indicates that the scaling policy status was changed by a user. <code>PROVISION_FAILURE</code> indicates that the status change was because the policy failed to provision. <code>CLEANUP_FAILURE</code> indicates an error.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[doc="<p>A friendly, more verbose message that accompanies an automatic scaling policy state change.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[doc="<p>The status of an automatic scaling policy. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AutoScalingPolicyStatus {
    #[doc="<p>Indicates the status of the automatic scaling policy.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The reason for a change in status.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<AutoScalingPolicyStateChangeReason>,
}
#[doc="<p>Configuration of a bootstrap action.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct BootstrapActionConfig {
    #[doc="<p>The name of the bootstrap action.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The script run by the bootstrap action.</p>"]
    #[serde(rename="ScriptBootstrapAction")]
    pub script_bootstrap_action: ScriptBootstrapActionConfig,
}
impl BootstrapActionConfig {
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `BootstrapActionConfig.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `script_bootstrap_action`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `BootstrapActionConfig.script_bootstrap_action = value.into();`.
    pub fn script_bootstrap_action<ValueType: Into<ScriptBootstrapActionConfig>>(mut self,
                                                                                 value: ValueType)
                                                                                 -> Self {
        self.script_bootstrap_action = value.into();
        self
    }
    /// Returns a new instance of BootstrapActionConfig with optional fields set to `None`.
    pub fn new<NameType: Into<String>, ScriptBootstrapActionType: Into<ScriptBootstrapActionConfig>>
        (name: NameType,
         script_bootstrap_action: ScriptBootstrapActionType)
         -> BootstrapActionConfig {
        BootstrapActionConfig {
            name: name.into(),
            script_bootstrap_action: script_bootstrap_action.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Reports the configuration of a bootstrap action in a cluster (job flow).</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BootstrapActionDetail {
    #[doc="<p>A description of the bootstrap action.</p>"]
    #[serde(rename="BootstrapActionConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bootstrap_action_config: Option<BootstrapActionConfig>,
}
#[doc="<p>Specification of the status of a CancelSteps request. Available only in Amazon EMR version 4.8.0 and later, excluding version 5.0.0.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CancelStepsInfo {
    #[doc="<p>The reason for the failure if the CancelSteps request fails.</p>"]
    #[serde(rename="Reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[doc="<p>The status of a CancelSteps Request. The value may be SUBMITTED or FAILED.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The encrypted StepId of a step.</p>"]
    #[serde(rename="StepId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_id: Option<String>,
}
#[doc="<p>The input argument to the <a>CancelSteps</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CancelStepsInput {
    #[doc="<p>The <code>ClusterID</code> for which specified steps will be canceled. Use <a>RunJobFlow</a> and <a>ListClusters</a> to get ClusterIDs. </p>"]
    #[serde(rename="ClusterId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc="<p>The list of <code>StepIDs</code> to cancel. Use <a>ListSteps</a> to get steps and their states for the specified cluster.</p>"]
    #[serde(rename="StepIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_ids: Option<Vec<String>>,
}
impl CancelStepsInput {
    /// Sets `cluster_id`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CancelStepsInput.cluster_id = Some(value.into());`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = Some(value.into());
        self
    }
    /// Sets `step_ids`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CancelStepsInput.step_ids = Some(value.into());`.
    pub fn step_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.step_ids = Some(value.into());
        self
    }
    /// Returns a new instance of CancelStepsInput with optional fields set to `None`.
    pub fn new() -> CancelStepsInput {
        CancelStepsInput { ..Default::default() }
    }
}
#[doc="<p> The output for the <a>CancelSteps</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CancelStepsOutput {
    #[doc="<p>A list of <a>CancelStepsInfo</a>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>"]
    #[serde(rename="CancelStepsInfoList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cancel_steps_info_list: Option<Vec<CancelStepsInfo>>,
}
#[doc="<p>The definition of a CloudWatch metric alarm, which determines when an automatic scaling activity is triggered. When the defined alarm conditions are satisfied, scaling activity begins.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CloudWatchAlarmDefinition {
    #[doc="<p>Determines how the metric specified by <code>MetricName</code> is compared to the value specified by <code>Threshold</code>.</p>"]
    #[serde(rename="ComparisonOperator")]
    pub comparison_operator: String,
    #[doc="<p>A CloudWatch metric dimension.</p>"]
    #[serde(rename="Dimensions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[doc="<p>The number of periods, expressed in seconds using <code>Period</code>, during which the alarm condition must exist before the alarm triggers automatic scaling activity. The default value is <code>1</code>.</p>"]
    #[serde(rename="EvaluationPeriods")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub evaluation_periods: Option<i64>,
    #[doc="<p>The name of the CloudWatch metric that is watched to determine an alarm condition.</p>"]
    #[serde(rename="MetricName")]
    pub metric_name: String,
    #[doc="<p>The namespace for the CloudWatch metric. The default is <code>AWS/ElasticMapReduce</code>.</p>"]
    #[serde(rename="Namespace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub namespace: Option<String>,
    #[doc="<p>The period, in seconds, over which the statistic is applied. EMR CloudWatch metrics are emitted every five minutes (300 seconds), so if an EMR CloudWatch metric is specified, specify <code>300</code>.</p>"]
    #[serde(rename="Period")]
    pub period: i64,
    #[doc="<p>The statistic to apply to the metric associated with the alarm. The default is <code>AVERAGE</code>.</p>"]
    #[serde(rename="Statistic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statistic: Option<String>,
    #[doc="<p>The value against which the specified statistic is compared.</p>"]
    #[serde(rename="Threshold")]
    pub threshold: f64,
    #[doc="<p>The unit of measure associated with the CloudWatch metric being watched. The value specified for <code>Unit</code> must correspond to the units specified in the CloudWatch metric.</p>"]
    #[serde(rename="Unit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit: Option<String>,
}
impl CloudWatchAlarmDefinition {
    /// Sets `comparison_operator`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.comparison_operator = value.into();`.
    pub fn comparison_operator<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.comparison_operator = value.into();
        self
    }
    /// Sets `dimensions`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.dimensions = Some(value.into());`.
    pub fn dimensions<ValueType: Into<Vec<MetricDimension>>>(mut self, value: ValueType) -> Self {
        self.dimensions = Some(value.into());
        self
    }
    /// Sets `evaluation_periods`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.evaluation_periods = Some(value.into());`.
    pub fn evaluation_periods<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.evaluation_periods = Some(value.into());
        self
    }
    /// Sets `metric_name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.metric_name = value.into();`.
    pub fn metric_name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.metric_name = value.into();
        self
    }
    /// Sets `namespace`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.namespace = Some(value.into());`.
    pub fn namespace<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.namespace = Some(value.into());
        self
    }
    /// Sets `period`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.period = value.into();`.
    pub fn period<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.period = value.into();
        self
    }
    /// Sets `statistic`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.statistic = Some(value.into());`.
    pub fn statistic<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.statistic = Some(value.into());
        self
    }
    /// Sets `threshold`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.threshold = value.into();`.
    pub fn threshold<ValueType: Into<f64>>(mut self, value: ValueType) -> Self {
        self.threshold = value.into();
        self
    }
    /// Sets `unit`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CloudWatchAlarmDefinition.unit = Some(value.into());`.
    pub fn unit<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.unit = Some(value.into());
        self
    }
    /// Returns a new instance of CloudWatchAlarmDefinition with optional fields set to `None`.
    pub fn new<ComparisonOperatorType: Into<String>,
               MetricNameType: Into<String>,
               PeriodType: Into<i64>,
               ThresholdType: Into<f64>>
        (comparison_operator: ComparisonOperatorType,
         metric_name: MetricNameType,
         period: PeriodType,
         threshold: ThresholdType)
         -> CloudWatchAlarmDefinition {
        CloudWatchAlarmDefinition {
            comparison_operator: comparison_operator.into(),
            metric_name: metric_name.into(),
            period: period.into(),
            threshold: threshold.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The detailed description of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Cluster {
    #[doc="<p>The applications installed on this cluster.</p>"]
    #[serde(rename="Applications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[doc="<p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.</p>"]
    #[serde(rename="AutoScalingRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_role: Option<String>,
    #[doc="<p>Specifies whether the cluster should terminate after completing all steps.</p>"]
    #[serde(rename="AutoTerminate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_terminate: Option<bool>,
    #[doc="<p>Applies only to Amazon EMR releases 4.x and later. The list of Configurations supplied to the EMR cluster.</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>Available only in Amazon EMR version 5.7.0 and later. The ID of a custom Amazon EBS-backed Linux AMI if the cluster uses a custom AMI.</p>"]
    #[serde(rename="CustomAmiId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[doc="<p>The size, in GiB, of the EBS root device volume of the Linux AMI that is used for each EC2 instance. Available in Amazon EMR version 4.x and later.</p>"]
    #[serde(rename="EbsRootVolumeSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_root_volume_size: Option<i64>,
    #[doc="<p>Provides information about the EC2 instances in a cluster grouped by category. For example, key name, subnet ID, IAM instance profile, and so on.</p>"]
    #[serde(rename="Ec2InstanceAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_instance_attributes: Option<Ec2InstanceAttributes>,
    #[doc="<p>The unique identifier for the cluster.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note> <p>The instance group configuration of the cluster. A value of <code>INSTANCE_GROUP</code> indicates a uniform instance group configuration. A value of <code>INSTANCE_FLEET</code> indicates an instance fleets configuration.</p>"]
    #[serde(rename="InstanceCollectionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_collection_type: Option<String>,
    #[doc="<p>The path to the Amazon S3 location where logs for this cluster are stored.</p>"]
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_uri: Option<String>,
    #[doc="<p>The public DNS name of the master EC2 instance.</p>"]
    #[serde(rename="MasterPublicDnsName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_public_dns_name: Option<String>,
    #[doc="<p>The name of the cluster.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An approximation of the cost of the cluster, represented in m1.small/hours. This value is incremented one time for every hour an m1.small instance runs. Larger instances are weighted more, so an EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>"]
    #[serde(rename="NormalizedInstanceHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub normalized_instance_hours: Option<i64>,
    #[doc="<p>The release label for the Amazon EMR release.</p>"]
    #[serde(rename="ReleaseLabel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub release_label: Option<String>,
    #[doc="<p>Applies only when <code>CustomAmiID</code> is used. Specifies the type of updates that are applied from the Amazon Linux AMI package repositories when an instance boots using the AMI.</p>"]
    #[serde(rename="RepoUpgradeOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo_upgrade_on_boot: Option<String>,
    #[doc="<p>The AMI version requested for this cluster.</p>"]
    #[serde(rename="RequestedAmiVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_ami_version: Option<String>,
    #[doc="<p>The AMI version running on this cluster.</p>"]
    #[serde(rename="RunningAmiVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_ami_version: Option<String>,
    #[doc="<p>The way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> is available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>"]
    #[serde(rename="ScaleDownBehavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scale_down_behavior: Option<String>,
    #[doc="<p>The name of the security configuration applied to the cluster.</p>"]
    #[serde(rename="SecurityConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_configuration: Option<String>,
    #[doc="<p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>"]
    #[serde(rename="ServiceRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role: Option<String>,
    #[doc="<p>The current status details about the cluster.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<ClusterStatus>,
    #[doc="<p>A list of tags associated with a cluster.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[doc="<p>Indicates whether Amazon EMR will lock the cluster to prevent the EC2 instances from being terminated by an API call or user intervention, or in the event of a cluster error.</p>"]
    #[serde(rename="TerminationProtected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub termination_protected: Option<bool>,
    #[doc="<p>Indicates whether the cluster is visible to all IAM users of the AWS account associated with the cluster. If this value is set to <code>true</code>, all IAM users of that AWS account can view and manage the cluster if they have the proper policy permissions set. If this value is <code>false</code>, only the IAM user that created the cluster can view and manage it. This value can be changed using the <a>SetVisibleToAllUsers</a> action.</p>"]
    #[serde(rename="VisibleToAllUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}
#[doc="<p>The reason that the cluster changed to its current state.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ClusterStateChangeReason {
    #[doc="<p>The programmatic code for the state change reason.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[doc="<p>The descriptive message for the state change reason.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[doc="<p>The detailed status of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ClusterStatus {
    #[doc="<p>The current state of the cluster.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The reason for the cluster status change.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<ClusterStateChangeReason>,
    #[doc="<p>A timeline that represents the status of a cluster over the lifetime of the cluster.</p>"]
    #[serde(rename="Timeline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline: Option<ClusterTimeline>,
}
#[doc="<p>The summary description of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ClusterSummary {
    #[doc="<p>The unique identifier for the cluster.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The name of the cluster.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An approximation of the cost of the cluster, represented in m1.small/hours. This value is incremented one time for every hour an m1.small instance runs. Larger instances are weighted more, so an EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>"]
    #[serde(rename="NormalizedInstanceHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub normalized_instance_hours: Option<i64>,
    #[doc="<p>The details about the current status of the cluster.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<ClusterStatus>,
}
#[doc="<p>Represents the timeline of the cluster's lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ClusterTimeline {
    #[doc="<p>The creation date and time of the cluster.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The date and time when the cluster was terminated.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>The date and time when the cluster was ready to execute steps.</p>"]
    #[serde(rename="ReadyDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ready_date_time: Option<f64>,
}
#[doc="<p>An entity describing an executable that runs on a cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Command {
    #[doc="<p>Arguments for Amazon EMR to pass to the command for execution.</p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<Vec<String>>,
    #[doc="<p>The name of the command.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The Amazon S3 location of the command script.</p>"]
    #[serde(rename="ScriptPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub script_path: Option<String>,
}
#[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>An optional configuration specification to be used when provisioning cluster instances, which can include configurations for applications and software bundled with Amazon EMR. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see <a href=\"http://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-configure-apps.html\">Configuring Applications</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Configuration {
    #[doc="<p>The classification within a configuration.</p>"]
    #[serde(rename="Classification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classification: Option<String>,
    #[doc="<p>A list of additional configurations to apply within a configuration object.</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>A set of properties specified within a configuration classification.</p>"]
    #[serde(rename="Properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}
impl Configuration {
    /// Sets `classification`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Configuration.classification = Some(value.into());`.
    pub fn classification<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.classification = Some(value.into());
        self
    }
    /// Sets `configurations`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Configuration.configurations = Some(value.into());`.
    pub fn configurations<ValueType: Into<Vec<Configuration>>>(mut self, value: ValueType) -> Self {
        self.configurations = Some(value.into());
        self
    }
    /// Sets `properties`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Configuration.properties = Some(value.into());`.
pub fn properties<ValueType: Into<::std::collections::HashMap<String, String>>>(mut self, value: ValueType) -> Self{
        self.properties = Some(value.into());
        self
    }
    /// Returns a new instance of Configuration with optional fields set to `None`.
    pub fn new() -> Configuration {
        Configuration { ..Default::default() }
    }
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateSecurityConfigurationInput {
    #[doc="<p>The name of the security configuration.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The security configuration details in JSON format.</p>"]
    #[serde(rename="SecurityConfiguration")]
    pub security_configuration: String,
}
impl CreateSecurityConfigurationInput {
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CreateSecurityConfigurationInput.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `security_configuration`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `CreateSecurityConfigurationInput.security_configuration = value.into();`.
    pub fn security_configuration<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.security_configuration = value.into();
        self
    }
    /// Returns a new instance of CreateSecurityConfigurationInput with optional fields set to `None`.
    pub fn new<NameType: Into<String>, SecurityConfigurationType: Into<String>>
        (name: NameType,
         security_configuration: SecurityConfigurationType)
         -> CreateSecurityConfigurationInput {
        CreateSecurityConfigurationInput {
            name: name.into(),
            security_configuration: security_configuration.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateSecurityConfigurationOutput {
    #[doc="<p>The date and time the security configuration was created.</p>"]
    #[serde(rename="CreationDateTime")]
    pub creation_date_time: f64,
    #[doc="<p>The name of the security configuration.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSecurityConfigurationInput {
    #[doc="<p>The name of the security configuration.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}
impl DeleteSecurityConfigurationInput {
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DeleteSecurityConfigurationInput.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Returns a new instance of DeleteSecurityConfigurationInput with optional fields set to `None`.
    pub fn new<NameType: Into<String>>(name: NameType) -> DeleteSecurityConfigurationInput {
        DeleteSecurityConfigurationInput {
            name: name.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSecurityConfigurationOutput;

#[doc="<p>This input determines which cluster to describe.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeClusterInput {
    #[doc="<p>The identifier of the cluster to describe.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
}
impl DescribeClusterInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeClusterInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Returns a new instance of DescribeClusterInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>>(cluster_id: ClusterIdType) -> DescribeClusterInput {
        DescribeClusterInput {
            cluster_id: cluster_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output contains the description of the cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeClusterOutput {
    #[doc="<p>This output contains the details for the requested cluster.</p>"]
    #[serde(rename="Cluster")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster: Option<Cluster>,
}
#[doc="<p> The input for the <a>DescribeJobFlows</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeJobFlowsInput {
    #[doc="<p>Return only job flows created after this date and time.</p>"]
    #[serde(rename="CreatedAfter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_after: Option<f64>,
    #[doc="<p>Return only job flows created before this date and time.</p>"]
    #[serde(rename="CreatedBefore")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_before: Option<f64>,
    #[doc="<p>Return only job flows whose job flow ID is contained in this list.</p>"]
    #[serde(rename="JobFlowIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flow_ids: Option<Vec<String>>,
    #[doc="<p>Return only job flows whose state is contained in this list.</p>"]
    #[serde(rename="JobFlowStates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flow_states: Option<Vec<String>>,
}
impl DescribeJobFlowsInput {
    /// Sets `created_after`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeJobFlowsInput.created_after = Some(value.into());`.
    pub fn created_after<ValueType: Into<f64>>(mut self, value: ValueType) -> Self {
        self.created_after = Some(value.into());
        self
    }
    /// Sets `created_before`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeJobFlowsInput.created_before = Some(value.into());`.
    pub fn created_before<ValueType: Into<f64>>(mut self, value: ValueType) -> Self {
        self.created_before = Some(value.into());
        self
    }
    /// Sets `job_flow_ids`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeJobFlowsInput.job_flow_ids = Some(value.into());`.
    pub fn job_flow_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.job_flow_ids = Some(value.into());
        self
    }
    /// Sets `job_flow_states`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeJobFlowsInput.job_flow_states = Some(value.into());`.
    pub fn job_flow_states<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.job_flow_states = Some(value.into());
        self
    }
    /// Returns a new instance of DescribeJobFlowsInput with optional fields set to `None`.
    pub fn new() -> DescribeJobFlowsInput {
        DescribeJobFlowsInput { ..Default::default() }
    }
}
#[doc="<p> The output for the <a>DescribeJobFlows</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeJobFlowsOutput {
    #[doc="<p>A list of job flows matching the parameters supplied.</p>"]
    #[serde(rename="JobFlows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flows: Option<Vec<JobFlowDetail>>,
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeSecurityConfigurationInput {
    #[doc="<p>The name of the security configuration.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}
impl DescribeSecurityConfigurationInput {
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeSecurityConfigurationInput.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Returns a new instance of DescribeSecurityConfigurationInput with optional fields set to `None`.
    pub fn new<NameType: Into<String>>(name: NameType) -> DescribeSecurityConfigurationInput {
        DescribeSecurityConfigurationInput {
            name: name.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeSecurityConfigurationOutput {
    #[doc="<p>The date and time the security configuration was created</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The name of the security configuration.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The security configuration details in JSON format.</p>"]
    #[serde(rename="SecurityConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_configuration: Option<String>,
}
#[doc="<p>This input determines which step to describe.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStepInput {
    #[doc="<p>The identifier of the cluster with steps to describe.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The identifier of the step to describe.</p>"]
    #[serde(rename="StepId")]
    pub step_id: String,
}
impl DescribeStepInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeStepInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `step_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `DescribeStepInput.step_id = value.into();`.
    pub fn step_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.step_id = value.into();
        self
    }
    /// Returns a new instance of DescribeStepInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>, StepIdType: Into<String>>(cluster_id: ClusterIdType,
                                                                      step_id: StepIdType)
                                                                      -> DescribeStepInput {
        DescribeStepInput {
            cluster_id: cluster_id.into(),
            step_id: step_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output contains the description of the cluster step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStepOutput {
    #[doc="<p>The step details for the requested step identifier.</p>"]
    #[serde(rename="Step")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step: Option<Step>,
}
#[doc="<p>Configuration of requested EBS block device associated with the instance group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EbsBlockDevice {
    #[doc="<p>The device name that is exposed to the instance, such as /dev/sdh.</p>"]
    #[serde(rename="Device")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device: Option<String>,
    #[doc="<p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>"]
    #[serde(rename="VolumeSpecification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_specification: Option<VolumeSpecification>,
}
#[doc="<p>Configuration of requested EBS block device associated with the instance group with count of volumes that will be associated to every instance.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EbsBlockDeviceConfig {
    #[doc="<p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>"]
    #[serde(rename="VolumeSpecification")]
    pub volume_specification: VolumeSpecification,
    #[doc="<p>Number of EBS volumes with a specific volume configuration that will be associated with every instance in the instance group</p>"]
    #[serde(rename="VolumesPerInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volumes_per_instance: Option<i64>,
}
impl EbsBlockDeviceConfig {
    /// Sets `volume_specification`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `EbsBlockDeviceConfig.volume_specification = value.into();`.
    pub fn volume_specification<ValueType: Into<VolumeSpecification>>(mut self,
                                                                      value: ValueType)
                                                                      -> Self {
        self.volume_specification = value.into();
        self
    }
    /// Sets `volumes_per_instance`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `EbsBlockDeviceConfig.volumes_per_instance = Some(value.into());`.
    pub fn volumes_per_instance<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.volumes_per_instance = Some(value.into());
        self
    }
    /// Returns a new instance of EbsBlockDeviceConfig with optional fields set to `None`.
pub fn new<VolumeSpecificationType: Into<VolumeSpecification>>(volume_specification: VolumeSpecificationType) -> EbsBlockDeviceConfig{
        EbsBlockDeviceConfig {
            volume_specification: volume_specification.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The Amazon EBS configuration of a cluster instance.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EbsConfiguration {
    #[doc="<p>An array of Amazon EBS volume specifications attached to a cluster instance.</p>"]
    #[serde(rename="EbsBlockDeviceConfigs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,
    #[doc="<p>Indicates whether an Amazon EBS volume is EBS-optimized.</p>"]
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_optimized: Option<bool>,
}
impl EbsConfiguration {
    /// Sets `ebs_block_device_configs`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `EbsConfiguration.ebs_block_device_configs = Some(value.into());`.
    pub fn ebs_block_device_configs<ValueType: Into<Vec<EbsBlockDeviceConfig>>>(mut self,
                                                                                value: ValueType)
                                                                                -> Self {
        self.ebs_block_device_configs = Some(value.into());
        self
    }
    /// Sets `ebs_optimized`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `EbsConfiguration.ebs_optimized = Some(value.into());`.
    pub fn ebs_optimized<ValueType: Into<bool>>(mut self, value: ValueType) -> Self {
        self.ebs_optimized = Some(value.into());
        self
    }
    /// Returns a new instance of EbsConfiguration with optional fields set to `None`.
    pub fn new() -> EbsConfiguration {
        EbsConfiguration { ..Default::default() }
    }
}
#[doc="<p>EBS block device that's attached to an EC2 instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EbsVolume {
    #[doc="<p>The device name that is exposed to the instance, such as /dev/sdh.</p>"]
    #[serde(rename="Device")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device: Option<String>,
    #[doc="<p>The volume identifier of the EBS volume.</p>"]
    #[serde(rename="VolumeId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume_id: Option<String>,
}
#[doc="<p>Provides information about the EC2 instances in a cluster grouped by category. For example, key name, subnet ID, IAM instance profile, and so on.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Ec2InstanceAttributes {
    #[doc="<p>A list of additional Amazon EC2 security group IDs for the master node.</p>"]
    #[serde(rename="AdditionalMasterSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_master_security_groups: Option<Vec<String>>,
    #[doc="<p>A list of additional Amazon EC2 security group IDs for the slave nodes.</p>"]
    #[serde(rename="AdditionalSlaveSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    #[doc="<p>The Availability Zone in which the cluster will run. </p>"]
    #[serde(rename="Ec2AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_availability_zone: Option<String>,
    #[doc="<p>The name of the Amazon EC2 key pair to use when connecting with SSH into the master node as a user named \"hadoop\".</p>"]
    #[serde(rename="Ec2KeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_key_name: Option<String>,
    #[doc="<p>To launch the cluster in Amazon VPC, set this parameter to the identifier of the Amazon VPC subnet where you want the cluster to launch. If you do not specify this value, the cluster is launched in the normal AWS cloud, outside of a VPC.</p> <p>Amazon VPC currently does not support cluster compute quadruple extra large (cc1.4xlarge) instances. Thus, you cannot specify the cc1.4xlarge instance type for nodes of a cluster launched in a VPC.</p>"]
    #[serde(rename="Ec2SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_subnet_id: Option<String>,
    #[doc="<p>The identifier of the Amazon EC2 security group for the master node.</p>"]
    #[serde(rename="EmrManagedMasterSecurityGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,
    #[doc="<p>The identifier of the Amazon EC2 security group for the slave nodes.</p>"]
    #[serde(rename="EmrManagedSlaveSecurityGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
    #[doc="<p>The IAM role that was specified when the cluster was launched. The EC2 instances of the cluster assume this role.</p>"]
    #[serde(rename="IamInstanceProfile")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iam_instance_profile: Option<String>,
    #[doc="<p>Applies to clusters configured with the instance fleets option. Specifies one or more Availability Zones in which to launch EC2 cluster instances when the EC2-Classic network configuration is supported. Amazon EMR chooses the Availability Zone with the best fit from among the list of <code>RequestedEc2AvailabilityZones</code>, and then launches all cluster instances within that Availability Zone. If you do not specify this value, Amazon EMR chooses the Availability Zone for you. <code>RequestedEc2SubnetIDs</code> and <code>RequestedEc2AvailabilityZones</code> cannot be specified together.</p>"]
    #[serde(rename="RequestedEc2AvailabilityZones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_ec_2_availability_zones: Option<Vec<String>>,
    #[doc="<p>Applies to clusters configured with the instance fleets option. Specifies the unique identifier of one or more Amazon EC2 subnets in which to launch EC2 cluster instances. Subnets must exist within the same VPC. Amazon EMR chooses the EC2 subnet with the best fit from among the list of <code>RequestedEc2SubnetIds</code>, and then launches all cluster instances within that Subnet. If this value is not specified, and the account and region support EC2-Classic networks, the cluster launches instances in the EC2-Classic network and uses <code>RequestedEc2AvailabilityZones</code> instead of this setting. If EC2-Classic is not supported, and no Subnet is specified, Amazon EMR chooses the subnet for you. <code>RequestedEc2SubnetIDs</code> and <code>RequestedEc2AvailabilityZones</code> cannot be specified together.</p>"]
    #[serde(rename="RequestedEc2SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_ec_2_subnet_ids: Option<Vec<String>>,
    #[doc="<p>The identifier of the Amazon EC2 security group for the Amazon EMR service to access clusters in VPC private subnets.</p>"]
    #[serde(rename="ServiceAccessSecurityGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_access_security_group: Option<String>,
}
#[doc="<p>The details of the step failure. The service attempts to detect the root cause for many common failures.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FailureDetails {
    #[doc="<p>The path to the log file where the step failure root cause was originally recorded.</p>"]
    #[serde(rename="LogFile")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_file: Option<String>,
    #[doc="<p>The descriptive message including the error the EMR service has identified as the cause of step failure. This is text from an error log that describes the root cause of the failure.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p>The reason for the step failure. In the case where the service cannot successfully determine the root cause of the failure, it returns \"Unknown Error\" as a reason.</p>"]
    #[serde(rename="Reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
}
#[doc="<p>A job flow step consisting of a JAR file whose main function will be executed. The main function submits a job for Hadoop to execute and waits for the job to finish or fail.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct HadoopJarStepConfig {
    #[doc="<p>A list of command line arguments passed to the JAR file's main function when executed.</p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<Vec<String>>,
    #[doc="<p>A path to a JAR file run during the step.</p>"]
    #[serde(rename="Jar")]
    pub jar: String,
    #[doc="<p>The name of the main class in the specified Java file. If not specified, the JAR file should specify a Main-Class in its manifest file.</p>"]
    #[serde(rename="MainClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub main_class: Option<String>,
    #[doc="<p>A list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.</p>"]
    #[serde(rename="Properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<Vec<KeyValue>>,
}
impl HadoopJarStepConfig {
    /// Sets `args`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `HadoopJarStepConfig.args = Some(value.into());`.
    pub fn args<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.args = Some(value.into());
        self
    }
    /// Sets `jar`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `HadoopJarStepConfig.jar = value.into();`.
    pub fn jar<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.jar = value.into();
        self
    }
    /// Sets `main_class`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `HadoopJarStepConfig.main_class = Some(value.into());`.
    pub fn main_class<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.main_class = Some(value.into());
        self
    }
    /// Sets `properties`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `HadoopJarStepConfig.properties = Some(value.into());`.
    pub fn properties<ValueType: Into<Vec<KeyValue>>>(mut self, value: ValueType) -> Self {
        self.properties = Some(value.into());
        self
    }
    /// Returns a new instance of HadoopJarStepConfig with optional fields set to `None`.
    pub fn new<JarType: Into<String>>(jar: JarType) -> HadoopJarStepConfig {
        HadoopJarStepConfig {
            jar: jar.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>A cluster step consisting of a JAR file whose main function will be executed. The main function submits a job for Hadoop to execute and waits for the job to finish or fail.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct HadoopStepConfig {
    #[doc="<p>The list of command line arguments to pass to the JAR file's main function for execution.</p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<Vec<String>>,
    #[doc="<p>The path to the JAR file that runs during the step.</p>"]
    #[serde(rename="Jar")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jar: Option<String>,
    #[doc="<p>The name of the main class in the specified Java file. If not specified, the JAR file should specify a main class in its manifest file.</p>"]
    #[serde(rename="MainClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub main_class: Option<String>,
    #[doc="<p>The list of Java properties that are set when the step runs. You can use these properties to pass key value pairs to your main function.</p>"]
    #[serde(rename="Properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}
#[doc="<p>Represents an EC2 instance provisioned as part of cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Instance {
    #[doc="<p>The list of EBS volumes that are attached to this instance.</p>"]
    #[serde(rename="EbsVolumes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_volumes: Option<Vec<EbsVolume>>,
    #[doc="<p>The unique identifier of the instance in Amazon EC2.</p>"]
    #[serde(rename="Ec2InstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    #[doc="<p>The unique identifier for the instance in Amazon EMR.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The unique identifier of the instance fleet to which an EC2 instance belongs.</p>"]
    #[serde(rename="InstanceFleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleet_id: Option<String>,
    #[doc="<p>The identifier of the instance group to which this instance belongs.</p>"]
    #[serde(rename="InstanceGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_id: Option<String>,
    #[doc="<p>The EC2 instance type, for example <code>m3.xlarge</code>.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The instance purchasing option. Valid values are <code>ON_DEMAND</code> or <code>SPOT</code>. </p>"]
    #[serde(rename="Market")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub market: Option<String>,
    #[doc="<p>The private DNS name of the instance.</p>"]
    #[serde(rename="PrivateDnsName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_dns_name: Option<String>,
    #[doc="<p>The private IP address of the instance.</p>"]
    #[serde(rename="PrivateIpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc="<p>The public DNS name of the instance.</p>"]
    #[serde(rename="PublicDnsName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_dns_name: Option<String>,
    #[doc="<p>The public IP address of the instance.</p>"]
    #[serde(rename="PublicIpAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc="<p>The current status of the instance.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<InstanceStatus>,
}
#[doc="<p>Describes an instance fleet, which is a group of EC2 instances that host a particular node type (master, core, or task) in an Amazon EMR cluster. Instance fleets can consist of a mix of instance types and On-Demand and Spot instances, which are provisioned to meet a defined target capacity. </p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceFleet {
    #[doc="<p>The unique identifier of the instance fleet.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The node type that the instance fleet hosts. Valid values are MASTER, CORE, or TASK. </p>"]
    #[serde(rename="InstanceFleetType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleet_type: Option<String>,
    #[doc="<p>The specification for the instance types that comprise an instance fleet. Up to five unique instance specifications may be defined for each instance fleet. </p>"]
    #[serde(rename="InstanceTypeSpecifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type_specifications: Option<Vec<InstanceTypeSpecification>>,
    #[doc="<p>Describes the launch specification for an instance fleet. </p>"]
    #[serde(rename="LaunchSpecifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    #[doc="<p>A friendly name for the instance fleet.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The number of On-Demand units that have been provisioned for the instance fleet to fulfill <code>TargetOnDemandCapacity</code>. This provisioned capacity might be less than or greater than <code>TargetOnDemandCapacity</code>.</p>"]
    #[serde(rename="ProvisionedOnDemandCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_on_demand_capacity: Option<i64>,
    #[doc="<p>The number of Spot units that have been provisioned for this instance fleet to fulfill <code>TargetSpotCapacity</code>. This provisioned capacity might be less than or greater than <code>TargetSpotCapacity</code>.</p>"]
    #[serde(rename="ProvisionedSpotCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provisioned_spot_capacity: Option<i64>,
    #[doc="<p>The current status of the instance fleet. </p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<InstanceFleetStatus>,
    #[doc="<p>The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision. When the instance fleet launches, Amazon EMR tries to provision On-Demand instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When an On-Demand instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units. You can use <a>InstanceFleet$ProvisionedOnDemandCapacity</a> to determine the Spot capacity units that have been provisioned for the instance fleet.</p> <note> <p>If not specified or set to 0, only Spot instances are provisioned for the instance fleet using <code>TargetSpotCapacity</code>. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note>"]
    #[serde(rename="TargetOnDemandCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_on_demand_capacity: Option<i64>,
    #[doc="<p>The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision. When the instance fleet launches, Amazon EMR tries to provision Spot instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When a Spot instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units. You can use <a>InstanceFleet$ProvisionedSpotCapacity</a> to determine the Spot capacity units that have been provisioned for the instance fleet.</p> <note> <p>If not specified or set to 0, only On-Demand instances are provisioned for the instance fleet. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note>"]
    #[serde(rename="TargetSpotCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_spot_capacity: Option<i64>,
}
#[doc="<p>The configuration that defines an instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InstanceFleetConfig {
    #[doc="<p>The node type that the instance fleet hosts. Valid values are MASTER,CORE,and TASK.</p>"]
    #[serde(rename="InstanceFleetType")]
    pub instance_fleet_type: String,
    #[doc="<p>The instance type configurations that define the EC2 instances in the instance fleet.</p>"]
    #[serde(rename="InstanceTypeConfigs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type_configs: Option<Vec<InstanceTypeConfig>>,
    #[doc="<p>The launch specification for the instance fleet.</p>"]
    #[serde(rename="LaunchSpecifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub launch_specifications: Option<InstanceFleetProvisioningSpecifications>,
    #[doc="<p>The friendly name of the instance fleet.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision. When the instance fleet launches, Amazon EMR tries to provision On-Demand instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When an On-Demand instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units.</p> <note> <p>If not specified or set to 0, only Spot instances are provisioned for the instance fleet using <code>TargetSpotCapacity</code>. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note>"]
    #[serde(rename="TargetOnDemandCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_on_demand_capacity: Option<i64>,
    #[doc="<p>The target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision. When the instance fleet launches, Amazon EMR tries to provision Spot instances as specified by <a>InstanceTypeConfig</a>. Each instance configuration has a specified <code>WeightedCapacity</code>. When a Spot instance is provisioned, the <code>WeightedCapacity</code> units count toward the target capacity. Amazon EMR provisions instances until the target capacity is totally fulfilled, even if this results in an overage. For example, if there are 2 units remaining to fulfill capacity, and Amazon EMR can only provision an instance with a <code>WeightedCapacity</code> of 5 units, the instance is provisioned, and the target capacity is exceeded by 3 units.</p> <note> <p>If not specified or set to 0, only On-Demand instances are provisioned for the instance fleet. At least one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> should be greater than 0. For a master instance fleet, only one of <code>TargetSpotCapacity</code> and <code>TargetOnDemandCapacity</code> can be specified, and its value must be 1.</p> </note>"]
    #[serde(rename="TargetSpotCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_spot_capacity: Option<i64>,
}
impl InstanceFleetConfig {
    /// Sets `instance_fleet_type`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetConfig.instance_fleet_type = value.into();`.
    pub fn instance_fleet_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_fleet_type = value.into();
        self
    }
    /// Sets `instance_type_configs`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetConfig.instance_type_configs = Some(value.into());`.
    pub fn instance_type_configs<ValueType: Into<Vec<InstanceTypeConfig>>>(mut self,
                                                                           value: ValueType)
                                                                           -> Self {
        self.instance_type_configs = Some(value.into());
        self
    }
    /// Sets `launch_specifications`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetConfig.launch_specifications = Some(value.into());`.
    pub fn launch_specifications<ValueType: Into<InstanceFleetProvisioningSpecifications>>
        (mut self,
         value: ValueType)
         -> Self {
        self.launch_specifications = Some(value.into());
        self
    }
    /// Sets `name`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetConfig.name = Some(value.into());`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets `target_on_demand_capacity`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetConfig.target_on_demand_capacity = Some(value.into());`.
    pub fn target_on_demand_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.target_on_demand_capacity = Some(value.into());
        self
    }
    /// Sets `target_spot_capacity`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetConfig.target_spot_capacity = Some(value.into());`.
    pub fn target_spot_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.target_spot_capacity = Some(value.into());
        self
    }
    /// Returns a new instance of InstanceFleetConfig with optional fields set to `None`.
    pub fn new<InstanceFleetTypeType: Into<String>>(instance_fleet_type: InstanceFleetTypeType)
                                                    -> InstanceFleetConfig {
        InstanceFleetConfig {
            instance_fleet_type: instance_fleet_type.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Configuration parameters for an instance fleet modification request.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InstanceFleetModifyConfig {
    #[doc="<p>A unique identifier for the instance fleet.</p>"]
    #[serde(rename="InstanceFleetId")]
    pub instance_fleet_id: String,
    #[doc="<p>The target capacity of On-Demand units for the instance fleet. For more information see <a>InstanceFleetConfig$TargetOnDemandCapacity</a>.</p>"]
    #[serde(rename="TargetOnDemandCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_on_demand_capacity: Option<i64>,
    #[doc="<p>The target capacity of Spot units for the instance fleet. For more information, see <a>InstanceFleetConfig$TargetSpotCapacity</a>.</p>"]
    #[serde(rename="TargetSpotCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_spot_capacity: Option<i64>,
}
impl InstanceFleetModifyConfig {
    /// Sets `instance_fleet_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetModifyConfig.instance_fleet_id = value.into();`.
    pub fn instance_fleet_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_fleet_id = value.into();
        self
    }
    /// Sets `target_on_demand_capacity`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetModifyConfig.target_on_demand_capacity = Some(value.into());`.
    pub fn target_on_demand_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.target_on_demand_capacity = Some(value.into());
        self
    }
    /// Sets `target_spot_capacity`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetModifyConfig.target_spot_capacity = Some(value.into());`.
    pub fn target_spot_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.target_spot_capacity = Some(value.into());
        self
    }
    /// Returns a new instance of InstanceFleetModifyConfig with optional fields set to `None`.
    pub fn new<InstanceFleetIdType: Into<String>>(instance_fleet_id: InstanceFleetIdType)
                                                  -> InstanceFleetModifyConfig {
        InstanceFleetModifyConfig {
            instance_fleet_id: instance_fleet_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The launch specification for Spot instances in the fleet, which determines the defined duration and provisioning timeout behavior.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct InstanceFleetProvisioningSpecifications {
    #[doc="<p>The launch specification for Spot instances in the fleet, which determines the defined duration and provisioning timeout behavior.</p>"]
    #[serde(rename="SpotSpecification")]
    pub spot_specification: SpotProvisioningSpecification,
}
impl InstanceFleetProvisioningSpecifications {
    /// Sets `spot_specification`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceFleetProvisioningSpecifications.spot_specification = value.into();`.
    pub fn spot_specification<ValueType: Into<SpotProvisioningSpecification>>(mut self,
                                                                              value: ValueType)
                                                                              -> Self {
        self.spot_specification = value.into();
        self
    }
    /// Returns a new instance of InstanceFleetProvisioningSpecifications with optional fields set to `None`.
    pub fn new<SpotSpecificationType: Into<SpotProvisioningSpecification>>
        (spot_specification: SpotSpecificationType)
         -> InstanceFleetProvisioningSpecifications {
        InstanceFleetProvisioningSpecifications {
            spot_specification: spot_specification.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Provides status change reason details for the instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceFleetStateChangeReason {
    #[doc="<p>A code corresponding to the reason the state change occurred.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[doc="<p>An explanatory message.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[doc="<p>The status of the instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceFleetStatus {
    #[doc="<p>A code representing the instance fleet status.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>Provides status change reason details for the instance fleet.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<InstanceFleetStateChangeReason>,
    #[doc="<p>Provides historical timestamps for the instance fleet, including the time of creation, the time it became ready to run jobs, and the time of termination.</p>"]
    #[serde(rename="Timeline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline: Option<InstanceFleetTimeline>,
}
#[doc="<p>Provides historical timestamps for the instance fleet, including the time of creation, the time it became ready to run jobs, and the time of termination.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceFleetTimeline {
    #[doc="<p>The time and date the instance fleet was created.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The time and date the instance fleet terminated.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>The time and date the instance fleet was ready to run jobs.</p>"]
    #[serde(rename="ReadyDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ready_date_time: Option<f64>,
}
#[doc="<p>This entity represents an instance group, which is a group of instances that have common purpose. For example, CORE instance group is used for HDFS.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceGroup {
    #[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See PutAutoScalingPolicy.</p>"]
    #[serde(rename="AutoScalingPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
    #[doc="<p>The bid price for each EC2 instance in the instance group when launching nodes as Spot Instances, expressed in USD.</p>"]
    #[serde(rename="BidPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price: Option<String>,
    #[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for an EMR cluster instance group. You can specify a separate configuration for each instance group (master, core, and task).</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>The EBS block devices that are mapped to this instance group.</p>"]
    #[serde(rename="EbsBlockDevices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_block_devices: Option<Vec<EbsBlockDevice>>,
    #[doc="<p>If the instance group is EBS-optimized. An Amazon EBS-optimized instance uses an optimized configuration stack and provides additional, dedicated capacity for Amazon EBS I/O.</p>"]
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[doc="<p>The identifier of the instance group.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The type of the instance group. Valid values are MASTER, CORE or TASK.</p>"]
    #[serde(rename="InstanceGroupType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_type: Option<String>,
    #[doc="<p>The EC2 instance type for all instances in the instance group.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The marketplace to provision instances for this group. Valid values are ON_DEMAND or SPOT.</p>"]
    #[serde(rename="Market")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub market: Option<String>,
    #[doc="<p>The name of the instance group.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The target number of instances for the instance group.</p>"]
    #[serde(rename="RequestedInstanceCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_instance_count: Option<i64>,
    #[doc="<p>The number of instances currently running in this instance group.</p>"]
    #[serde(rename="RunningInstanceCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub running_instance_count: Option<i64>,
    #[doc="<p>Policy for customizing shrink operations.</p>"]
    #[serde(rename="ShrinkPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shrink_policy: Option<ShrinkPolicy>,
    #[doc="<p>The current status of the instance group.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<InstanceGroupStatus>,
}
#[doc="<p>Configuration defining a new instance group.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InstanceGroupConfig {
    #[doc="<p>An automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. See <a>PutAutoScalingPolicy</a>.</p>"]
    #[serde(rename="AutoScalingPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,
    #[doc="<p>Bid price for each EC2 instance in the instance group when launching nodes as Spot Instances, expressed in USD.</p>"]
    #[serde(rename="BidPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price: Option<String>,
    #[doc="<note> <p>Amazon EMR releases 4.x or later.</p> </note> <p>The list of configurations supplied for an EMR cluster instance group. You can specify a separate configuration for each instance group (master, core, and task).</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>EBS configurations that will be attached to each EC2 instance in the instance group.</p>"]
    #[serde(rename="EbsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[doc="<p>Target number of instances for the instance group.</p>"]
    #[serde(rename="InstanceCount")]
    pub instance_count: i64,
    #[doc="<p>The role of the instance group in the cluster.</p>"]
    #[serde(rename="InstanceRole")]
    pub instance_role: String,
    #[doc="<p>The EC2 instance type for all instances in the instance group.</p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p>Market type of the EC2 instances used to create a cluster node.</p>"]
    #[serde(rename="Market")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub market: Option<String>,
    #[doc="<p>Friendly name given to the instance group.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}
impl InstanceGroupConfig {
    /// Sets `auto_scaling_policy`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.auto_scaling_policy = Some(value.into());`.
    pub fn auto_scaling_policy<ValueType: Into<AutoScalingPolicy>>(mut self,
                                                                   value: ValueType)
                                                                   -> Self {
        self.auto_scaling_policy = Some(value.into());
        self
    }
    /// Sets `bid_price`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.bid_price = Some(value.into());`.
    pub fn bid_price<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.bid_price = Some(value.into());
        self
    }
    /// Sets `configurations`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.configurations = Some(value.into());`.
    pub fn configurations<ValueType: Into<Vec<Configuration>>>(mut self, value: ValueType) -> Self {
        self.configurations = Some(value.into());
        self
    }
    /// Sets `ebs_configuration`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.ebs_configuration = Some(value.into());`.
    pub fn ebs_configuration<ValueType: Into<EbsConfiguration>>(mut self,
                                                                value: ValueType)
                                                                -> Self {
        self.ebs_configuration = Some(value.into());
        self
    }
    /// Sets `instance_count`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.instance_count = value.into();`.
    pub fn instance_count<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.instance_count = value.into();
        self
    }
    /// Sets `instance_role`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.instance_role = value.into();`.
    pub fn instance_role<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_role = value.into();
        self
    }
    /// Sets `instance_type`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.instance_type = value.into();`.
    pub fn instance_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_type = value.into();
        self
    }
    /// Sets `market`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.market = Some(value.into());`.
    pub fn market<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.market = Some(value.into());
        self
    }
    /// Sets `name`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupConfig.name = Some(value.into());`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Returns a new instance of InstanceGroupConfig with optional fields set to `None`.
    pub fn new<InstanceCountType: Into<i64>,
               InstanceRoleType: Into<String>,
               InstanceTypeType: Into<String>>
        (instance_count: InstanceCountType,
         instance_role: InstanceRoleType,
         instance_type: InstanceTypeType)
         -> InstanceGroupConfig {
        InstanceGroupConfig {
            instance_count: instance_count.into(),
            instance_role: instance_role.into(),
            instance_type: instance_type.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Detailed information about an instance group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceGroupDetail {
    #[doc="<p>Bid price for EC2 Instances when launching nodes as Spot Instances, expressed in USD.</p>"]
    #[serde(rename="BidPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price: Option<String>,
    #[doc="<p>The date/time the instance group was created.</p>"]
    #[serde(rename="CreationDateTime")]
    pub creation_date_time: f64,
    #[doc="<p>The date/time the instance group was terminated.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>Unique identifier for the instance group.</p>"]
    #[serde(rename="InstanceGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_id: Option<String>,
    #[doc="<p>Target number of instances to run in the instance group.</p>"]
    #[serde(rename="InstanceRequestCount")]
    pub instance_request_count: i64,
    #[doc="<p>Instance group role in the cluster</p>"]
    #[serde(rename="InstanceRole")]
    pub instance_role: String,
    #[doc="<p>Actual count of running instances.</p>"]
    #[serde(rename="InstanceRunningCount")]
    pub instance_running_count: i64,
    #[doc="<p>EC2 instance type.</p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p>Details regarding the state of the instance group.</p>"]
    #[serde(rename="LastStateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[doc="<p>Market type of the EC2 instances used to create a cluster node.</p>"]
    #[serde(rename="Market")]
    pub market: String,
    #[doc="<p>Friendly name for the instance group.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The date/time the instance group was available to the cluster.</p>"]
    #[serde(rename="ReadyDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ready_date_time: Option<f64>,
    #[doc="<p>The date/time the instance group was started.</p>"]
    #[serde(rename="StartDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_date_time: Option<f64>,
    #[doc="<p>State of instance group. The following values are deprecated: STARTING, TERMINATED, and FAILED.</p>"]
    #[serde(rename="State")]
    pub state: String,
}
#[doc="<p>Modify an instance group size.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InstanceGroupModifyConfig {
    #[doc="<p>The EC2 InstanceIds to terminate. After you terminate the instances, the instance group will not return to its original requested size.</p>"]
    #[serde(rename="EC2InstanceIdsToTerminate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec2_instance_ids_to_terminate: Option<Vec<String>>,
    #[doc="<p>Target size for the instance group.</p>"]
    #[serde(rename="InstanceCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_count: Option<i64>,
    #[doc="<p>Unique ID of the instance group to expand or shrink.</p>"]
    #[serde(rename="InstanceGroupId")]
    pub instance_group_id: String,
    #[doc="<p>Policy for customizing shrink operations.</p>"]
    #[serde(rename="ShrinkPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shrink_policy: Option<ShrinkPolicy>,
}
impl InstanceGroupModifyConfig {
    /// Sets `ec2_instance_ids_to_terminate`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupModifyConfig.ec2_instance_ids_to_terminate = Some(value.into());`.
    pub fn ec2_instance_ids_to_terminate<ValueType: Into<Vec<String>>>(mut self,
                                                                       value: ValueType)
                                                                       -> Self {
        self.ec2_instance_ids_to_terminate = Some(value.into());
        self
    }
    /// Sets `instance_count`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupModifyConfig.instance_count = Some(value.into());`.
    pub fn instance_count<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.instance_count = Some(value.into());
        self
    }
    /// Sets `instance_group_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupModifyConfig.instance_group_id = value.into();`.
    pub fn instance_group_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_group_id = value.into();
        self
    }
    /// Sets `shrink_policy`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceGroupModifyConfig.shrink_policy = Some(value.into());`.
    pub fn shrink_policy<ValueType: Into<ShrinkPolicy>>(mut self, value: ValueType) -> Self {
        self.shrink_policy = Some(value.into());
        self
    }
    /// Returns a new instance of InstanceGroupModifyConfig with optional fields set to `None`.
    pub fn new<InstanceGroupIdType: Into<String>>(instance_group_id: InstanceGroupIdType)
                                                  -> InstanceGroupModifyConfig {
        InstanceGroupModifyConfig {
            instance_group_id: instance_group_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The status change reason details for the instance group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceGroupStateChangeReason {
    #[doc="<p>The programmable code for the state change reason.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[doc="<p>The status change reason description.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[doc="<p>The details of the instance group status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceGroupStatus {
    #[doc="<p>The current state of the instance group.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The status change reason details for the instance group.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<InstanceGroupStateChangeReason>,
    #[doc="<p>The timeline of the instance group status over time.</p>"]
    #[serde(rename="Timeline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline: Option<InstanceGroupTimeline>,
}
#[doc="<p>The timeline of the instance group lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceGroupTimeline {
    #[doc="<p>The creation date and time of the instance group.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The date and time when the instance group terminated.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>The date and time when the instance group became ready to perform tasks.</p>"]
    #[serde(rename="ReadyDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ready_date_time: Option<f64>,
}
#[doc="<p>Custom policy for requesting termination protection or termination of specific instances when shrinking an instance group.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct InstanceResizePolicy {
    #[doc="<p>Decommissioning timeout override for the specific list of instances to be terminated.</p>"]
    #[serde(rename="InstanceTerminationTimeout")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_termination_timeout: Option<i64>,
    #[doc="<p>Specific list of instances to be protected when shrinking an instance group.</p>"]
    #[serde(rename="InstancesToProtect")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances_to_protect: Option<Vec<String>>,
    #[doc="<p>Specific list of instances to be terminated when shrinking an instance group.</p>"]
    #[serde(rename="InstancesToTerminate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances_to_terminate: Option<Vec<String>>,
}
impl InstanceResizePolicy {
    /// Sets `instance_termination_timeout`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceResizePolicy.instance_termination_timeout = Some(value.into());`.
    pub fn instance_termination_timeout<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.instance_termination_timeout = Some(value.into());
        self
    }
    /// Sets `instances_to_protect`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceResizePolicy.instances_to_protect = Some(value.into());`.
    pub fn instances_to_protect<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.instances_to_protect = Some(value.into());
        self
    }
    /// Sets `instances_to_terminate`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceResizePolicy.instances_to_terminate = Some(value.into());`.
    pub fn instances_to_terminate<ValueType: Into<Vec<String>>>(mut self,
                                                                value: ValueType)
                                                                -> Self {
        self.instances_to_terminate = Some(value.into());
        self
    }
    /// Returns a new instance of InstanceResizePolicy with optional fields set to `None`.
    pub fn new() -> InstanceResizePolicy {
        InstanceResizePolicy { ..Default::default() }
    }
}
#[doc="<p>The details of the status change reason for the instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceStateChangeReason {
    #[doc="<p>The programmable code for the state change reason.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[doc="<p>The status change reason description.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[doc="<p>The instance status details.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceStatus {
    #[doc="<p>The current state of the instance.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The details of the status change reason for the instance.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<InstanceStateChangeReason>,
    #[doc="<p>The timeline of the instance status over time.</p>"]
    #[serde(rename="Timeline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline: Option<InstanceTimeline>,
}
#[doc="<p>The timeline of the instance lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceTimeline {
    #[doc="<p>The creation date and time of the instance.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The date and time when the instance was terminated.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>The date and time when the instance was ready to perform tasks.</p>"]
    #[serde(rename="ReadyDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ready_date_time: Option<f64>,
}
#[doc="<p>An instance type configuration for each instance type in an instance fleet, which determines the EC2 instances Amazon EMR attempts to provision to fulfill On-Demand and Spot target capacities. There can be a maximum of 5 instance type configurations in a fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InstanceTypeConfig {
    #[doc="<p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD. If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%. </p>"]
    #[serde(rename="BidPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price: Option<String>,
    #[doc="<p>The bid price, as a percentage of On-Demand price, for each EC2 Spot instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%). If neither <code>BidPrice</code> nor <code>BidPriceAsPercentageOfOnDemandPrice</code> is provided, <code>BidPriceAsPercentageOfOnDemandPrice</code> defaults to 100%.</p>"]
    #[serde(rename="BidPriceAsPercentageOfOnDemandPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    #[doc="<p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software that run on the cluster.</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>The configuration of Amazon Elastic Block Storage (EBS) attached to each instance as defined by <code>InstanceType</code>. </p>"]
    #[serde(rename="EbsConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_configuration: Option<EbsConfiguration>,
    #[doc="<p>An EC2 instance type, such as <code>m3.xlarge</code>. </p>"]
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[doc="<p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <a>InstanceFleetConfig</a>. This value is 1 for a master instance fleet, and must be 1 or greater for core and task instance fleets. Defaults to 1 if not specified. </p>"]
    #[serde(rename="WeightedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub weighted_capacity: Option<i64>,
}
impl InstanceTypeConfig {
    /// Sets `bid_price`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceTypeConfig.bid_price = Some(value.into());`.
    pub fn bid_price<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.bid_price = Some(value.into());
        self
    }
    /// Sets `bid_price_as_percentage_of_on_demand_price`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceTypeConfig.bid_price_as_percentage_of_on_demand_price = Some(value.into());`.
    pub fn bid_price_as_percentage_of_on_demand_price<ValueType: Into<f64>>(mut self,
                                                                            value: ValueType)
                                                                            -> Self {
        self.bid_price_as_percentage_of_on_demand_price = Some(value.into());
        self
    }
    /// Sets `configurations`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceTypeConfig.configurations = Some(value.into());`.
    pub fn configurations<ValueType: Into<Vec<Configuration>>>(mut self, value: ValueType) -> Self {
        self.configurations = Some(value.into());
        self
    }
    /// Sets `ebs_configuration`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceTypeConfig.ebs_configuration = Some(value.into());`.
    pub fn ebs_configuration<ValueType: Into<EbsConfiguration>>(mut self,
                                                                value: ValueType)
                                                                -> Self {
        self.ebs_configuration = Some(value.into());
        self
    }
    /// Sets `instance_type`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceTypeConfig.instance_type = value.into();`.
    pub fn instance_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_type = value.into();
        self
    }
    /// Sets `weighted_capacity`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `InstanceTypeConfig.weighted_capacity = Some(value.into());`.
    pub fn weighted_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.weighted_capacity = Some(value.into());
        self
    }
    /// Returns a new instance of InstanceTypeConfig with optional fields set to `None`.
    pub fn new<InstanceTypeType: Into<String>>(instance_type: InstanceTypeType)
                                               -> InstanceTypeConfig {
        InstanceTypeConfig {
            instance_type: instance_type.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The configuration specification for each instance type in an instance fleet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct InstanceTypeSpecification {
    #[doc="<p>The bid price for each EC2 Spot instance type as defined by <code>InstanceType</code>. Expressed in USD.</p>"]
    #[serde(rename="BidPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price: Option<String>,
    #[doc="<p>The bid price, as a percentage of On-Demand price, for each EC2 Spot instance as defined by <code>InstanceType</code>. Expressed as a number (for example, 20 specifies 20%).</p>"]
    #[serde(rename="BidPriceAsPercentageOfOnDemandPrice")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
    #[doc="<p>A configuration classification that applies when provisioning cluster instances, which can include configurations for applications and software bundled with Amazon EMR.</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>The configuration of Amazon Elastic Block Storage (EBS) attached to each instance as defined by <code>InstanceType</code>.</p>"]
    #[serde(rename="EbsBlockDevices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_block_devices: Option<Vec<EbsBlockDevice>>,
    #[doc="<p>Evaluates to <code>TRUE</code> when the specified <code>InstanceType</code> is EBS-optimized.</p>"]
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[doc="<p>The EC2 instance type, for example <code>m3.xlarge</code>.</p>"]
    #[serde(rename="InstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_type: Option<String>,
    #[doc="<p>The number of units that a provisioned instance of this type provides toward fulfilling the target capacities defined in <a>InstanceFleetConfig</a>. Capacity values represent performance characteristics such as vCPUs, memory, or I/O. If not specified, the default value is 1.</p>"]
    #[serde(rename="WeightedCapacity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub weighted_capacity: Option<i64>,
}
#[doc="<p>A description of a cluster (job flow).</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobFlowDetail {
    #[doc="<p>Used only for version 2.x and 3.x of Amazon EMR. The version of the AMI used to initialize Amazon EC2 instances in the job flow. For a list of AMI versions supported by Amazon EMR, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/EnvironmentConfig_AMIVersion.html#ami-versions-supported\">AMI Versions Supported in EMR</a> in the <i>Amazon EMR Developer Guide.</i> </p>"]
    #[serde(rename="AmiVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ami_version: Option<String>,
    #[doc="<p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides a way for the automatic scaling feature to get the required permissions it needs to launch and terminate EC2 instances in an instance group.</p>"]
    #[serde(rename="AutoScalingRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_role: Option<String>,
    #[doc="<p>A list of the bootstrap actions run by the job flow.</p>"]
    #[serde(rename="BootstrapActions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bootstrap_actions: Option<Vec<BootstrapActionDetail>>,
    #[doc="<p>Describes the execution status of the job flow.</p>"]
    #[serde(rename="ExecutionStatusDetail")]
    pub execution_status_detail: JobFlowExecutionStatusDetail,
    #[doc="<p>Describes the Amazon EC2 instances of the job flow.</p>"]
    #[serde(rename="Instances")]
    pub instances: JobFlowInstancesDetail,
    #[doc="<p>The job flow identifier.</p>"]
    #[serde(rename="JobFlowId")]
    pub job_flow_id: String,
    #[doc="<p>The IAM role that was specified when the job flow was launched. The EC2 instances of the job flow assume this role.</p>"]
    #[serde(rename="JobFlowRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flow_role: Option<String>,
    #[doc="<p>The location in Amazon S3 where log files for the job are stored.</p>"]
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_uri: Option<String>,
    #[doc="<p>The name of the job flow.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>"]
    #[serde(rename="ScaleDownBehavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scale_down_behavior: Option<String>,
    #[doc="<p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>"]
    #[serde(rename="ServiceRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role: Option<String>,
    #[doc="<p>A list of steps run by the job flow.</p>"]
    #[serde(rename="Steps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub steps: Option<Vec<StepDetail>>,
    #[doc="<p>A list of strings set by third party software when the job flow is launched. If you are not using third party software to manage the job flow this value is empty.</p>"]
    #[serde(rename="SupportedProducts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_products: Option<Vec<String>>,
    #[doc="<p>Specifies whether the cluster is visible to all IAM users of the AWS account associated with the cluster. If this value is set to <code>true</code>, all IAM users of that AWS account can view and (if they have the proper policy permissions set) manage the cluster. If it is set to <code>false</code>, only the IAM user that created the cluster can view and manage it. This value can be changed using the <a>SetVisibleToAllUsers</a> action.</p>"]
    #[serde(rename="VisibleToAllUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}
#[doc="<p>Describes the status of the cluster (job flow).</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobFlowExecutionStatusDetail {
    #[doc="<p>The creation date and time of the job flow.</p>"]
    #[serde(rename="CreationDateTime")]
    pub creation_date_time: f64,
    #[doc="<p>The completion date and time of the job flow.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>Description of the job flow last changed state.</p>"]
    #[serde(rename="LastStateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[doc="<p>The date and time when the job flow was ready to start running bootstrap actions.</p>"]
    #[serde(rename="ReadyDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ready_date_time: Option<f64>,
    #[doc="<p>The start date and time of the job flow.</p>"]
    #[serde(rename="StartDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_date_time: Option<f64>,
    #[doc="<p>The state of the job flow.</p>"]
    #[serde(rename="State")]
    pub state: String,
}
#[doc="<p>A description of the Amazon EC2 instance on which the cluster (job flow) runs. A valid JobFlowInstancesConfig must contain either InstanceGroups or InstanceFleets, which is the recommended configuration. They cannot be used together. You may also have MasterInstanceType, SlaveInstanceType, and InstanceCount (all three must be present), but we don't recommend this configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct JobFlowInstancesConfig {
    #[doc="<p>A list of additional Amazon EC2 security group IDs for the master node.</p>"]
    #[serde(rename="AdditionalMasterSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_master_security_groups: Option<Vec<String>>,
    #[doc="<p>A list of additional Amazon EC2 security group IDs for the slave nodes.</p>"]
    #[serde(rename="AdditionalSlaveSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_slave_security_groups: Option<Vec<String>>,
    #[doc="<p>The name of the EC2 key pair that can be used to ssh to the master node as the user called \"hadoop.\"</p>"]
    #[serde(rename="Ec2KeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_key_name: Option<String>,
    #[doc="<p>Applies to clusters that use the uniform instance group configuration. To launch the cluster in Amazon Virtual Private Cloud (Amazon VPC), set this parameter to the identifier of the Amazon VPC subnet where you want the cluster to launch. If you do not specify this value, the cluster launches in the normal Amazon Web Services cloud, outside of an Amazon VPC, if the account launching the cluster supports EC2 Classic networks in the region where the cluster launches.</p> <p>Amazon VPC currently does not support cluster compute quadruple extra large (cc1.4xlarge) instances. Thus you cannot specify the cc1.4xlarge instance type for clusters launched in an Amazon VPC.</p>"]
    #[serde(rename="Ec2SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_subnet_id: Option<String>,
    #[doc="<p>Applies to clusters that use the instance fleet configuration. When multiple EC2 subnet IDs are specified, Amazon EMR evaluates them and launches instances in the optimal subnet.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
    #[serde(rename="Ec2SubnetIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_subnet_ids: Option<Vec<String>>,
    #[doc="<p>The identifier of the Amazon EC2 security group for the master node.</p>"]
    #[serde(rename="EmrManagedMasterSecurityGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emr_managed_master_security_group: Option<String>,
    #[doc="<p>The identifier of the Amazon EC2 security group for the slave nodes.</p>"]
    #[serde(rename="EmrManagedSlaveSecurityGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emr_managed_slave_security_group: Option<String>,
    #[doc="<p>The Hadoop version for the cluster. Valid inputs are \"0.18\" (deprecated), \"0.20\" (deprecated), \"0.20.205\" (deprecated), \"1.0.3\", \"2.2.0\", or \"2.4.0\". If you do not set this value, the default of 0.18 is used, unless the AmiVersion parameter is set in the RunJobFlow call, in which case the default version of Hadoop for that AMI version is used.</p>"]
    #[serde(rename="HadoopVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hadoop_version: Option<String>,
    #[doc="<p>The number of EC2 instances in the cluster.</p>"]
    #[serde(rename="InstanceCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_count: Option<i64>,
    #[doc="<note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note> <p>Describes the EC2 instances and instance configurations for clusters that use the instance fleet configuration.</p>"]
    #[serde(rename="InstanceFleets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleets: Option<Vec<InstanceFleetConfig>>,
    #[doc="<p>Configuration for the instance groups in a cluster.</p>"]
    #[serde(rename="InstanceGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupConfig>>,
    #[doc="<p>Specifies whether the cluster should remain available after completing all steps.</p>"]
    #[serde(rename="KeepJobFlowAliveWhenNoSteps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    #[doc="<p>The EC2 instance type of the master node.</p>"]
    #[serde(rename="MasterInstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_instance_type: Option<String>,
    #[doc="<p>The Availability Zone in which the cluster runs.</p>"]
    #[serde(rename="Placement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement: Option<PlacementType>,
    #[doc="<p>The identifier of the Amazon EC2 security group for the Amazon EMR service to access clusters in VPC private subnets.</p>"]
    #[serde(rename="ServiceAccessSecurityGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_access_security_group: Option<String>,
    #[doc="<p>The EC2 instance type of the slave nodes.</p>"]
    #[serde(rename="SlaveInstanceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slave_instance_type: Option<String>,
    #[doc="<p>Specifies whether to lock the cluster to prevent the Amazon EC2 instances from being terminated by API call, user intervention, or in the event of a job-flow error.</p>"]
    #[serde(rename="TerminationProtected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub termination_protected: Option<bool>,
}
impl JobFlowInstancesConfig {
    /// Sets `additional_master_security_groups`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.additional_master_security_groups = Some(value.into());`.
    pub fn additional_master_security_groups<ValueType: Into<Vec<String>>>(mut self,
                                                                           value: ValueType)
                                                                           -> Self {
        self.additional_master_security_groups = Some(value.into());
        self
    }
    /// Sets `additional_slave_security_groups`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.additional_slave_security_groups = Some(value.into());`.
    pub fn additional_slave_security_groups<ValueType: Into<Vec<String>>>(mut self,
                                                                          value: ValueType)
                                                                          -> Self {
        self.additional_slave_security_groups = Some(value.into());
        self
    }
    /// Sets `ec_2_key_name`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.ec_2_key_name = Some(value.into());`.
    pub fn ec_2_key_name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.ec_2_key_name = Some(value.into());
        self
    }
    /// Sets `ec_2_subnet_id`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.ec_2_subnet_id = Some(value.into());`.
    pub fn ec_2_subnet_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.ec_2_subnet_id = Some(value.into());
        self
    }
    /// Sets `ec_2_subnet_ids`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.ec_2_subnet_ids = Some(value.into());`.
    pub fn ec_2_subnet_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.ec_2_subnet_ids = Some(value.into());
        self
    }
    /// Sets `emr_managed_master_security_group`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.emr_managed_master_security_group = Some(value.into());`.
    pub fn emr_managed_master_security_group<ValueType: Into<String>>(mut self,
                                                                      value: ValueType)
                                                                      -> Self {
        self.emr_managed_master_security_group = Some(value.into());
        self
    }
    /// Sets `emr_managed_slave_security_group`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.emr_managed_slave_security_group = Some(value.into());`.
    pub fn emr_managed_slave_security_group<ValueType: Into<String>>(mut self,
                                                                     value: ValueType)
                                                                     -> Self {
        self.emr_managed_slave_security_group = Some(value.into());
        self
    }
    /// Sets `hadoop_version`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.hadoop_version = Some(value.into());`.
    pub fn hadoop_version<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.hadoop_version = Some(value.into());
        self
    }
    /// Sets `instance_count`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.instance_count = Some(value.into());`.
    pub fn instance_count<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.instance_count = Some(value.into());
        self
    }
    /// Sets `instance_fleets`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.instance_fleets = Some(value.into());`.
    pub fn instance_fleets<ValueType: Into<Vec<InstanceFleetConfig>>>(mut self,
                                                                      value: ValueType)
                                                                      -> Self {
        self.instance_fleets = Some(value.into());
        self
    }
    /// Sets `instance_groups`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.instance_groups = Some(value.into());`.
    pub fn instance_groups<ValueType: Into<Vec<InstanceGroupConfig>>>(mut self,
                                                                      value: ValueType)
                                                                      -> Self {
        self.instance_groups = Some(value.into());
        self
    }
    /// Sets `keep_job_flow_alive_when_no_steps`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.keep_job_flow_alive_when_no_steps = Some(value.into());`.
    pub fn keep_job_flow_alive_when_no_steps<ValueType: Into<bool>>(mut self,
                                                                    value: ValueType)
                                                                    -> Self {
        self.keep_job_flow_alive_when_no_steps = Some(value.into());
        self
    }
    /// Sets `master_instance_type`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.master_instance_type = Some(value.into());`.
    pub fn master_instance_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.master_instance_type = Some(value.into());
        self
    }
    /// Sets `placement`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.placement = Some(value.into());`.
    pub fn placement<ValueType: Into<PlacementType>>(mut self, value: ValueType) -> Self {
        self.placement = Some(value.into());
        self
    }
    /// Sets `service_access_security_group`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.service_access_security_group = Some(value.into());`.
    pub fn service_access_security_group<ValueType: Into<String>>(mut self,
                                                                  value: ValueType)
                                                                  -> Self {
        self.service_access_security_group = Some(value.into());
        self
    }
    /// Sets `slave_instance_type`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.slave_instance_type = Some(value.into());`.
    pub fn slave_instance_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.slave_instance_type = Some(value.into());
        self
    }
    /// Sets `termination_protected`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `JobFlowInstancesConfig.termination_protected = Some(value.into());`.
    pub fn termination_protected<ValueType: Into<bool>>(mut self, value: ValueType) -> Self {
        self.termination_protected = Some(value.into());
        self
    }
    /// Returns a new instance of JobFlowInstancesConfig with optional fields set to `None`.
    pub fn new() -> JobFlowInstancesConfig {
        JobFlowInstancesConfig { ..Default::default() }
    }
}
#[doc="<p>Specify the type of Amazon EC2 instances that the cluster (job flow) runs on.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct JobFlowInstancesDetail {
    #[doc="<p>The name of an Amazon EC2 key pair that can be used to ssh to the master node.</p>"]
    #[serde(rename="Ec2KeyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_key_name: Option<String>,
    #[doc="<p>For clusters launched within Amazon Virtual Private Cloud, this is the identifier of the subnet where the cluster was launched.</p>"]
    #[serde(rename="Ec2SubnetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ec_2_subnet_id: Option<String>,
    #[doc="<p>The Hadoop version for the cluster.</p>"]
    #[serde(rename="HadoopVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hadoop_version: Option<String>,
    #[doc="<p>The number of Amazon EC2 instances in the cluster. If the value is 1, the same instance serves as both the master and slave node. If the value is greater than 1, one instance is the master node and all others are slave nodes.</p>"]
    #[serde(rename="InstanceCount")]
    pub instance_count: i64,
    #[doc="<p>Details about the instance groups in a cluster.</p>"]
    #[serde(rename="InstanceGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupDetail>>,
    #[doc="<p>Specifies whether the cluster should remain available after completing all steps.</p>"]
    #[serde(rename="KeepJobFlowAliveWhenNoSteps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keep_job_flow_alive_when_no_steps: Option<bool>,
    #[doc="<p>The Amazon EC2 instance identifier of the master node.</p>"]
    #[serde(rename="MasterInstanceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_instance_id: Option<String>,
    #[doc="<p>The Amazon EC2 master node instance type.</p>"]
    #[serde(rename="MasterInstanceType")]
    pub master_instance_type: String,
    #[doc="<p>The DNS name of the master node.</p>"]
    #[serde(rename="MasterPublicDnsName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_public_dns_name: Option<String>,
    #[doc="<p>An approximation of the cost of the cluster, represented in m1.small/hours. This value is incremented one time for every hour that an m1.small runs. Larger instances are weighted more, so an Amazon EC2 instance that is roughly four times more expensive would result in the normalized instance hours being incremented by four. This result is only an approximation and does not reflect the actual billing rate.</p>"]
    #[serde(rename="NormalizedInstanceHours")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub normalized_instance_hours: Option<i64>,
    #[doc="<p>The Amazon EC2 Availability Zone for the cluster.</p>"]
    #[serde(rename="Placement")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placement: Option<PlacementType>,
    #[doc="<p>The Amazon EC2 slave node instance type.</p>"]
    #[serde(rename="SlaveInstanceType")]
    pub slave_instance_type: String,
    #[doc="<p>Specifies whether the Amazon EC2 instances in the cluster are protected from termination by API calls, user intervention, or in the event of a job-flow error.</p>"]
    #[serde(rename="TerminationProtected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub termination_protected: Option<bool>,
}
#[doc="<p>A key value pair.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct KeyValue {
    #[doc="<p>The unique identifier of a key value pair.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="<p>The value part of the identified key.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}
impl KeyValue {
    /// Sets `key`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `KeyValue.key = Some(value.into());`.
    pub fn key<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets `value`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `KeyValue.value = Some(value.into());`.
    pub fn value<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Returns a new instance of KeyValue with optional fields set to `None`.
    pub fn new() -> KeyValue {
        KeyValue { ..Default::default() }
    }
}
#[doc="<p>This input determines which bootstrap actions to retrieve.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListBootstrapActionsInput {
    #[doc="<p>The cluster identifier for the bootstrap actions to list.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
impl ListBootstrapActionsInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListBootstrapActionsInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListBootstrapActionsInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Returns a new instance of ListBootstrapActionsInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>>(cluster_id: ClusterIdType)
                                            -> ListBootstrapActionsInput {
        ListBootstrapActionsInput {
            cluster_id: cluster_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output contains the bootstrap actions detail.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListBootstrapActionsOutput {
    #[doc="<p>The bootstrap actions associated with the cluster.</p>"]
    #[serde(rename="BootstrapActions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bootstrap_actions: Option<Vec<Command>>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
#[doc="<p>This input determines how the ListClusters action filters the list of clusters that it returns.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListClustersInput {
    #[doc="<p>The cluster state filters to apply when listing clusters.</p>"]
    #[serde(rename="ClusterStates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_states: Option<Vec<String>>,
    #[doc="<p>The creation date and time beginning value filter for listing clusters.</p>"]
    #[serde(rename="CreatedAfter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_after: Option<f64>,
    #[doc="<p>The creation date and time end value filter for listing clusters.</p>"]
    #[serde(rename="CreatedBefore")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_before: Option<f64>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
impl ListClustersInput {
    /// Sets `cluster_states`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListClustersInput.cluster_states = Some(value.into());`.
    pub fn cluster_states<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.cluster_states = Some(value.into());
        self
    }
    /// Sets `created_after`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListClustersInput.created_after = Some(value.into());`.
    pub fn created_after<ValueType: Into<f64>>(mut self, value: ValueType) -> Self {
        self.created_after = Some(value.into());
        self
    }
    /// Sets `created_before`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListClustersInput.created_before = Some(value.into());`.
    pub fn created_before<ValueType: Into<f64>>(mut self, value: ValueType) -> Self {
        self.created_before = Some(value.into());
        self
    }
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListClustersInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Returns a new instance of ListClustersInput with optional fields set to `None`.
    pub fn new() -> ListClustersInput {
        ListClustersInput { ..Default::default() }
    }
}
#[doc="<p>This contains a ClusterSummaryList with the cluster details; for example, the cluster IDs, names, and status.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListClustersOutput {
    #[doc="<p>The list of clusters for the account based on the given filters.</p>"]
    #[serde(rename="Clusters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clusters: Option<Vec<ClusterSummary>>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListInstanceFleetsInput {
    #[doc="<p>The unique identifier of the cluster.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
impl ListInstanceFleetsInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstanceFleetsInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstanceFleetsInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Returns a new instance of ListInstanceFleetsInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>>(cluster_id: ClusterIdType) -> ListInstanceFleetsInput {
        ListInstanceFleetsInput {
            cluster_id: cluster_id.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListInstanceFleetsOutput {
    #[doc="<p>The list of instance fleets for the cluster and given filters.</p>"]
    #[serde(rename="InstanceFleets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleets: Option<Vec<InstanceFleet>>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
#[doc="<p>This input determines which instance groups to retrieve.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListInstanceGroupsInput {
    #[doc="<p>The identifier of the cluster for which to list the instance groups.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
impl ListInstanceGroupsInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstanceGroupsInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstanceGroupsInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Returns a new instance of ListInstanceGroupsInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>>(cluster_id: ClusterIdType) -> ListInstanceGroupsInput {
        ListInstanceGroupsInput {
            cluster_id: cluster_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This input determines which instance groups to retrieve.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListInstanceGroupsOutput {
    #[doc="<p>The list of instance groups for the cluster and given filters.</p>"]
    #[serde(rename="InstanceGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroup>>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
#[doc="<p>This input determines which instances to list.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListInstancesInput {
    #[doc="<p>The identifier of the cluster for which to list the instances.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The unique identifier of the instance fleet.</p>"]
    #[serde(rename="InstanceFleetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleet_id: Option<String>,
    #[doc="<p>The node type of the instance fleet. For example MASTER, CORE, or TASK.</p>"]
    #[serde(rename="InstanceFleetType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_fleet_type: Option<String>,
    #[doc="<p>The identifier of the instance group for which to list the instances.</p>"]
    #[serde(rename="InstanceGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_id: Option<String>,
    #[doc="<p>The type of instance group for which to list the instances.</p>"]
    #[serde(rename="InstanceGroupTypes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_types: Option<Vec<String>>,
    #[doc="<p>A list of instance states that will filter the instances returned with this request.</p>"]
    #[serde(rename="InstanceStates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_states: Option<Vec<String>>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
impl ListInstancesInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `instance_fleet_id`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.instance_fleet_id = Some(value.into());`.
    pub fn instance_fleet_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_fleet_id = Some(value.into());
        self
    }
    /// Sets `instance_fleet_type`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.instance_fleet_type = Some(value.into());`.
    pub fn instance_fleet_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_fleet_type = Some(value.into());
        self
    }
    /// Sets `instance_group_id`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.instance_group_id = Some(value.into());`.
    pub fn instance_group_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_group_id = Some(value.into());
        self
    }
    /// Sets `instance_group_types`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.instance_group_types = Some(value.into());`.
    pub fn instance_group_types<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.instance_group_types = Some(value.into());
        self
    }
    /// Sets `instance_states`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.instance_states = Some(value.into());`.
    pub fn instance_states<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.instance_states = Some(value.into());
        self
    }
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListInstancesInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Returns a new instance of ListInstancesInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>>(cluster_id: ClusterIdType) -> ListInstancesInput {
        ListInstancesInput {
            cluster_id: cluster_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output contains the list of instances.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListInstancesOutput {
    #[doc="<p>The list of instances for the cluster and given filters.</p>"]
    #[serde(rename="Instances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListSecurityConfigurationsInput {
    #[doc="<p>The pagination token that indicates the set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}
impl ListSecurityConfigurationsInput {
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListSecurityConfigurationsInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Returns a new instance of ListSecurityConfigurationsInput with optional fields set to `None`.
    pub fn new() -> ListSecurityConfigurationsInput {
        ListSecurityConfigurationsInput { ..Default::default() }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListSecurityConfigurationsOutput {
    #[doc="<p>A pagination token that indicates the next set of results to retrieve. Include the marker in the next ListSecurityConfiguration call to retrieve the next page of results, if required.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The creation date and time, and name, of each security configuration.</p>"]
    #[serde(rename="SecurityConfigurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfigurationSummary>>,
}
#[doc="<p>This input determines which steps to list.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListStepsInput {
    #[doc="<p>The identifier of the cluster for which to list the steps.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The filter to limit the step list based on the identifier of the steps.</p>"]
    #[serde(rename="StepIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_ids: Option<Vec<String>>,
    #[doc="<p>The filter to limit the step list based on certain states.</p>"]
    #[serde(rename="StepStates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub step_states: Option<Vec<String>>,
}
impl ListStepsInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListStepsInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `marker`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListStepsInput.marker = Some(value.into());`.
    pub fn marker<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.marker = Some(value.into());
        self
    }
    /// Sets `step_ids`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListStepsInput.step_ids = Some(value.into());`.
    pub fn step_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.step_ids = Some(value.into());
        self
    }
    /// Sets `step_states`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ListStepsInput.step_states = Some(value.into());`.
    pub fn step_states<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.step_states = Some(value.into());
        self
    }
    /// Returns a new instance of ListStepsInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>>(cluster_id: ClusterIdType) -> ListStepsInput {
        ListStepsInput {
            cluster_id: cluster_id.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output contains the list of steps returned in reverse order. This means that the last step is the first element in the list.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListStepsOutput {
    #[doc="<p>The pagination token that indicates the next set of results to retrieve.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The filtered list of steps for the cluster.</p>"]
    #[serde(rename="Steps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub steps: Option<Vec<StepSummary>>,
}
#[doc="<p>A CloudWatch dimension, which is specified using a <code>Key</code> (known as a <code>Name</code> in CloudWatch), <code>Value</code> pair. By default, Amazon EMR uses one dimension whose <code>Key</code> is <code>JobFlowID</code> and <code>Value</code> is a variable representing the cluster ID, which is <code>${emr.clusterId}</code>. This enables the rule to bootstrap when the cluster ID becomes available.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MetricDimension {
    #[doc="<p>The dimension name.</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="<p>The dimension value.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}
impl MetricDimension {
    /// Sets `key`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `MetricDimension.key = Some(value.into());`.
    pub fn key<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets `value`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `MetricDimension.value = Some(value.into());`.
    pub fn value<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Returns a new instance of MetricDimension with optional fields set to `None`.
    pub fn new() -> MetricDimension {
        MetricDimension { ..Default::default() }
    }
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyInstanceFleetInput {
    #[doc="<p>The unique identifier of the cluster.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>The unique identifier of the instance fleet.</p>"]
    #[serde(rename="InstanceFleet")]
    pub instance_fleet: InstanceFleetModifyConfig,
}
impl ModifyInstanceFleetInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ModifyInstanceFleetInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `instance_fleet`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ModifyInstanceFleetInput.instance_fleet = value.into();`.
    pub fn instance_fleet<ValueType: Into<InstanceFleetModifyConfig>>(mut self,
                                                                      value: ValueType)
                                                                      -> Self {
        self.instance_fleet = value.into();
        self
    }
    /// Returns a new instance of ModifyInstanceFleetInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>, InstanceFleetType: Into<InstanceFleetModifyConfig>>
        (cluster_id: ClusterIdType,
         instance_fleet: InstanceFleetType)
         -> ModifyInstanceFleetInput {
        ModifyInstanceFleetInput {
            cluster_id: cluster_id.into(),
            instance_fleet: instance_fleet.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Change the size of some instance groups.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyInstanceGroupsInput {
    #[doc="<p>The ID of the cluster to which the instance group belongs.</p>"]
    #[serde(rename="ClusterId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc="<p>Instance groups to change.</p>"]
    #[serde(rename="InstanceGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_groups: Option<Vec<InstanceGroupModifyConfig>>,
}
impl ModifyInstanceGroupsInput {
    /// Sets `cluster_id`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ModifyInstanceGroupsInput.cluster_id = Some(value.into());`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = Some(value.into());
        self
    }
    /// Sets `instance_groups`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ModifyInstanceGroupsInput.instance_groups = Some(value.into());`.
    pub fn instance_groups<ValueType: Into<Vec<InstanceGroupModifyConfig>>>(mut self,
                                                                            value: ValueType)
                                                                            -> Self {
        self.instance_groups = Some(value.into());
        self
    }
    /// Returns a new instance of ModifyInstanceGroupsInput with optional fields set to `None`.
    pub fn new() -> ModifyInstanceGroupsInput {
        ModifyInstanceGroupsInput { ..Default::default() }
    }
}
#[doc="<p>The Amazon EC2 Availability Zone configuration of the cluster (job flow).</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PlacementType {
    #[doc="<p>The Amazon EC2 Availability Zone for the cluster. <code>AvailabilityZone</code> is used for uniform instance groups, while <code>AvailabilityZones</code> (plural) is used for instance fleets.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>When multiple Availability Zones are specified, Amazon EMR evaluates them and launches instances in the optimal Availability Zone. <code>AvailabilityZones</code> is used for instance fleets, while <code>AvailabilityZone</code> (singular) is used for uniform instance groups.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
    #[serde(rename="AvailabilityZones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
}
impl PlacementType {
    /// Sets `availability_zone`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `PlacementType.availability_zone = Some(value.into());`.
    pub fn availability_zone<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.availability_zone = Some(value.into());
        self
    }
    /// Sets `availability_zones`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `PlacementType.availability_zones = Some(value.into());`.
    pub fn availability_zones<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.availability_zones = Some(value.into());
        self
    }
    /// Returns a new instance of PlacementType with optional fields set to `None`.
    pub fn new() -> PlacementType {
        PlacementType { ..Default::default() }
    }
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutAutoScalingPolicyInput {
    #[doc="<p>Specifies the definition of the automatic scaling policy.</p>"]
    #[serde(rename="AutoScalingPolicy")]
    pub auto_scaling_policy: AutoScalingPolicy,
    #[doc="<p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>Specifies the ID of the instance group to which the automatic scaling policy is applied.</p>"]
    #[serde(rename="InstanceGroupId")]
    pub instance_group_id: String,
}
impl PutAutoScalingPolicyInput {
    /// Sets `auto_scaling_policy`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `PutAutoScalingPolicyInput.auto_scaling_policy = value.into();`.
    pub fn auto_scaling_policy<ValueType: Into<AutoScalingPolicy>>(mut self,
                                                                   value: ValueType)
                                                                   -> Self {
        self.auto_scaling_policy = value.into();
        self
    }
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `PutAutoScalingPolicyInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `instance_group_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `PutAutoScalingPolicyInput.instance_group_id = value.into();`.
    pub fn instance_group_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_group_id = value.into();
        self
    }
    /// Returns a new instance of PutAutoScalingPolicyInput with optional fields set to `None`.
    pub fn new<AutoScalingPolicyType: Into<AutoScalingPolicy>,
               ClusterIdType: Into<String>,
               InstanceGroupIdType: Into<String>>
        (auto_scaling_policy: AutoScalingPolicyType,
         cluster_id: ClusterIdType,
         instance_group_id: InstanceGroupIdType)
         -> PutAutoScalingPolicyInput {
        PutAutoScalingPolicyInput {
            auto_scaling_policy: auto_scaling_policy.into(),
            cluster_id: cluster_id.into(),
            instance_group_id: instance_group_id.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutAutoScalingPolicyOutput {
    #[doc="<p>The automatic scaling policy definition.</p>"]
    #[serde(rename="AutoScalingPolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicyDescription>,
    #[doc="<p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>"]
    #[serde(rename="ClusterId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc="<p>Specifies the ID of the instance group to which the scaling policy is applied.</p>"]
    #[serde(rename="InstanceGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_group_id: Option<String>,
}
#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveAutoScalingPolicyInput {
    #[doc="<p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>"]
    #[serde(rename="ClusterId")]
    pub cluster_id: String,
    #[doc="<p>Specifies the ID of the instance group to which the scaling policy is applied.</p>"]
    #[serde(rename="InstanceGroupId")]
    pub instance_group_id: String,
}
impl RemoveAutoScalingPolicyInput {
    /// Sets `cluster_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RemoveAutoScalingPolicyInput.cluster_id = value.into();`.
    pub fn cluster_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.cluster_id = value.into();
        self
    }
    /// Sets `instance_group_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RemoveAutoScalingPolicyInput.instance_group_id = value.into();`.
    pub fn instance_group_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.instance_group_id = value.into();
        self
    }
    /// Returns a new instance of RemoveAutoScalingPolicyInput with optional fields set to `None`.
    pub fn new<ClusterIdType: Into<String>, InstanceGroupIdType: Into<String>>
        (cluster_id: ClusterIdType,
         instance_group_id: InstanceGroupIdType)
         -> RemoveAutoScalingPolicyInput {
        RemoveAutoScalingPolicyInput {
            cluster_id: cluster_id.into(),
            instance_group_id: instance_group_id.into(),
            ..Default::default()
        }
    }
}
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveAutoScalingPolicyOutput;

#[doc="<p>This input identifies a cluster and a list of tags to remove.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveTagsInput {
    #[doc="<p>The Amazon EMR resource identifier from which tags will be removed. This value must be a cluster identifier.</p>"]
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    #[doc="<p>A list of tag keys to remove from a resource.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: Vec<String>,
}
impl RemoveTagsInput {
    /// Sets `resource_id`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RemoveTagsInput.resource_id = value.into();`.
    pub fn resource_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.resource_id = value.into();
        self
    }
    /// Sets `tag_keys`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RemoveTagsInput.tag_keys = value.into();`.
    pub fn tag_keys<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.tag_keys = value.into();
        self
    }
    /// Returns a new instance of RemoveTagsInput with optional fields set to `None`.
pub fn new<ResourceIdType: Into<String>, TagKeysType: Into<Vec<String>>>(resource_id: ResourceIdType, tag_keys: TagKeysType) -> RemoveTagsInput{
        RemoveTagsInput {
            resource_id: resource_id.into(),
            tag_keys: tag_keys.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This output indicates the result of removing tags from a resource.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveTagsOutput;

#[doc="<p> Input to the <a>RunJobFlow</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RunJobFlowInput {
    #[doc="<p>A JSON string for selecting additional features.</p>"]
    #[serde(rename="AdditionalInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_info: Option<String>,
    #[doc="<p>For Amazon EMR AMI versions 3.x and 2.x. For Amazon EMR releases 4.0 and later, the Linux AMI is determined by the <code>ReleaseLabel</code> specified or by <code>CustomAmiID</code>. The version of the Amazon Machine Image (AMI) to use when launching Amazon EC2 instances in the job flow. For details about the AMI versions currently supported in EMR version 3.x and 2.x, see <a href=\"ElasticMapReduce/latest/DeveloperGuide/emr-dg.pdf#nameddest=ami-versions-supported\">AMI Versions Supported in EMR</a> in the <i>Amazon EMR Developer Guide</i>. </p> <p>If the AMI supports multiple versions of Hadoop (for example, AMI 1.0 supports both Hadoop 0.18 and 0.20), you can use the <a>JobFlowInstancesConfig</a> <code>HadoopVersion</code> parameter to modify the version of Hadoop from the defaults shown above.</p> <note> <p>Previously, the EMR AMI version API parameter options allowed you to use latest for the latest AMI version rather than specify a numerical value. Some regions no longer support this deprecated option as they only have a newer release label version of EMR, which requires you to specify an EMR release label release (EMR 4.x or later).</p> </note>"]
    #[serde(rename="AmiVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ami_version: Option<String>,
    #[doc="<p>For Amazon EMR releases 4.0 and later. A list of applications for the cluster. Valid values are: \"Hadoop\", \"Hive\", \"Mahout\", \"Pig\", and \"Spark.\" They are case insensitive.</p>"]
    #[serde(rename="Applications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[doc="<p>An IAM role for automatic scaling policies. The default role is <code>EMR_AutoScaling_DefaultRole</code>. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.</p>"]
    #[serde(rename="AutoScalingRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_scaling_role: Option<String>,
    #[doc="<p>A list of bootstrap actions to run before Hadoop starts on the cluster nodes.</p>"]
    #[serde(rename="BootstrapActions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bootstrap_actions: Option<Vec<BootstrapActionConfig>>,
    #[doc="<p>For Amazon EMR releases 4.0 and later. The list of configurations supplied for the EMR cluster you are creating.</p>"]
    #[serde(rename="Configurations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[doc="<p>Available only in Amazon EMR version 5.7.0 and later. The ID of a custom Amazon EBS-backed Linux AMI. If specified, Amazon EMR uses this AMI when it launches cluster EC2 instances. For more information about custom AMIs in Amazon EMR, see <a href=\"http://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-custom-ami.html\">Using a Custom AMI</a> in the <i>Amazon EMR Management Guide</i>. If omitted, the cluster uses the base Linux AMI for the <code>ReleaseLabel</code> specified. For Amazon EMR versions 2.x and 3.x, use <code>AmiVersion</code> instead.</p> <p>For information about creating a custom AMI, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/creating-an-ami-ebs.html\">Creating an Amazon EBS-Backed Linux AMI</a> in the <i>Amazon Elastic Compute Cloud User Guide for Linux Instances</i>. For information about finding an AMI ID, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html\">Finding a Linux AMI</a>. </p>"]
    #[serde(rename="CustomAmiId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_ami_id: Option<String>,
    #[doc="<p>The size, in GiB, of the EBS root device volume of the Linux AMI that is used for each EC2 instance. Available in Amazon EMR version 4.x and later.</p>"]
    #[serde(rename="EbsRootVolumeSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ebs_root_volume_size: Option<i64>,
    #[doc="<p>A specification of the number and type of Amazon EC2 instances.</p>"]
    #[serde(rename="Instances")]
    pub instances: JobFlowInstancesConfig,
    #[doc="<p>Also called instance profile and EC2 role. An IAM role for an EMR cluster. The EC2 instances of the cluster assume this role. The default role is <code>EMR_EC2_DefaultRole</code>. In order to use the default role, you must have already created it using the CLI or console.</p>"]
    #[serde(rename="JobFlowRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flow_role: Option<String>,
    #[doc="<p>The location in Amazon S3 to write the log files of the job flow. If a value is not provided, logs are not created.</p>"]
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_uri: Option<String>,
    #[doc="<p>The name of the job flow.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and later, use Applications.</p> </note> <p>A list of strings that indicates third-party software to use with the job flow that accepts a user argument list. EMR accepts and forwards the argument list to the corresponding installation script as bootstrap action arguments. For more information, see \"Launch a Job Flow on the MapR Distribution for Hadoop\" in the <a href=\"http://docs.aws.amazon.com/http:/docs.aws.amazon.com/emr/latest/DeveloperGuide/emr-dg.pdf\">Amazon EMR Developer Guide</a>. Supported values are:</p> <ul> <li> <p>\"mapr-m3\" - launch the cluster using MapR M3 Edition.</p> </li> <li> <p>\"mapr-m5\" - launch the cluster using MapR M5 Edition.</p> </li> <li> <p>\"mapr\" with the user arguments specifying \"--edition,m3\" or \"--edition,m5\" - launch the job flow using MapR M3 or M5 Edition respectively.</p> </li> <li> <p>\"mapr-m7\" - launch the cluster using MapR M7 Edition.</p> </li> <li> <p>\"hunk\" - launch the cluster with the Hunk Big Data Analtics Platform.</p> </li> <li> <p>\"hue\"- launch the cluster with Hue installed.</p> </li> <li> <p>\"spark\" - launch the cluster with Apache Spark installed.</p> </li> <li> <p>\"ganglia\" - launch the cluster with the Ganglia Monitoring System installed.</p> </li> </ul>"]
    #[serde(rename="NewSupportedProducts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_supported_products: Option<Vec<SupportedProductConfig>>,
    #[doc="<p> The release label for the Amazon EMR release. For Amazon EMR 3.x and 2.x AMIs, use <code>AmiVersion</code> instead.</p>"]
    #[serde(rename="ReleaseLabel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub release_label: Option<String>,
    #[doc="<p>Applies only when <code>CustomAmiID</code> is used. Specifies which updates from the Amazon Linux AMI package repositories to apply automatically when the instance boots using the AMI. If omitted, the default is <code>SECURITY</code>, which indicates that only security updates are applied. If <code>NONE</code> is specified, no updates are applied, and all updates must be applied manually.</p>"]
    #[serde(rename="RepoUpgradeOnBoot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo_upgrade_on_boot: Option<String>,
    #[doc="<p>Specifies the way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an instance group is resized. <code>TERMINATE_AT_INSTANCE_HOUR</code> indicates that Amazon EMR terminates nodes at the instance-hour boundary, regardless of when the request to terminate the instance was submitted. This option is only available with Amazon EMR 5.1.0 and later and is the default for clusters created using that version. <code>TERMINATE_AT_TASK_COMPLETION</code> indicates that Amazon EMR blacklists and drains tasks from nodes before terminating the Amazon EC2 instances, regardless of the instance-hour boundary. With either behavior, Amazon EMR removes the least active nodes first and blocks instance termination if it could lead to HDFS corruption. <code>TERMINATE_AT_TASK_COMPLETION</code> available only in Amazon EMR version 4.1.0 and later, and is the default for versions of Amazon EMR earlier than 5.1.0.</p>"]
    #[serde(rename="ScaleDownBehavior")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scale_down_behavior: Option<String>,
    #[doc="<p>The name of a security configuration to apply to the cluster.</p>"]
    #[serde(rename="SecurityConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_configuration: Option<String>,
    #[doc="<p>The IAM role that will be assumed by the Amazon EMR service to access AWS resources on your behalf.</p>"]
    #[serde(rename="ServiceRole")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_role: Option<String>,
    #[doc="<p>A list of steps to run.</p>"]
    #[serde(rename="Steps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub steps: Option<Vec<StepConfig>>,
    #[doc="<note> <p>For Amazon EMR releases 3.x and 2.x. For Amazon EMR releases 4.x and later, use Applications.</p> </note> <p>A list of strings that indicates third-party software to use. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-supported-products.html\">Use Third Party Applications with Amazon EMR</a>. Currently supported values are:</p> <ul> <li> <p>\"mapr-m3\" - launch the job flow using MapR M3 Edition.</p> </li> <li> <p>\"mapr-m5\" - launch the job flow using MapR M5 Edition.</p> </li> </ul>"]
    #[serde(rename="SupportedProducts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_products: Option<Vec<String>>,
    #[doc="<p>A list of tags to associate with a cluster and propagate to Amazon EC2 instances.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[doc="<p>Whether the cluster is visible to all IAM users of the AWS account associated with the cluster. If this value is set to <code>true</code>, all IAM users of that AWS account can view and (if they have the proper policy permissions set) manage the cluster. If it is set to <code>false</code>, only the IAM user that created the cluster can view and manage it.</p>"]
    #[serde(rename="VisibleToAllUsers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visible_to_all_users: Option<bool>,
}
impl RunJobFlowInput {
    /// Sets `additional_info`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.additional_info = Some(value.into());`.
    pub fn additional_info<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.additional_info = Some(value.into());
        self
    }
    /// Sets `ami_version`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.ami_version = Some(value.into());`.
    pub fn ami_version<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.ami_version = Some(value.into());
        self
    }
    /// Sets `applications`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.applications = Some(value.into());`.
    pub fn applications<ValueType: Into<Vec<Application>>>(mut self, value: ValueType) -> Self {
        self.applications = Some(value.into());
        self
    }
    /// Sets `auto_scaling_role`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.auto_scaling_role = Some(value.into());`.
    pub fn auto_scaling_role<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.auto_scaling_role = Some(value.into());
        self
    }
    /// Sets `bootstrap_actions`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.bootstrap_actions = Some(value.into());`.
    pub fn bootstrap_actions<ValueType: Into<Vec<BootstrapActionConfig>>>(mut self,
                                                                          value: ValueType)
                                                                          -> Self {
        self.bootstrap_actions = Some(value.into());
        self
    }
    /// Sets `configurations`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.configurations = Some(value.into());`.
    pub fn configurations<ValueType: Into<Vec<Configuration>>>(mut self, value: ValueType) -> Self {
        self.configurations = Some(value.into());
        self
    }
    /// Sets `custom_ami_id`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.custom_ami_id = Some(value.into());`.
    pub fn custom_ami_id<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.custom_ami_id = Some(value.into());
        self
    }
    /// Sets `ebs_root_volume_size`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.ebs_root_volume_size = Some(value.into());`.
    pub fn ebs_root_volume_size<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.ebs_root_volume_size = Some(value.into());
        self
    }
    /// Sets `instances`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.instances = value.into();`.
    pub fn instances<ValueType: Into<JobFlowInstancesConfig>>(mut self, value: ValueType) -> Self {
        self.instances = value.into();
        self
    }
    /// Sets `job_flow_role`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.job_flow_role = Some(value.into());`.
    pub fn job_flow_role<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.job_flow_role = Some(value.into());
        self
    }
    /// Sets `log_uri`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.log_uri = Some(value.into());`.
    pub fn log_uri<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.log_uri = Some(value.into());
        self
    }
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `new_supported_products`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.new_supported_products = Some(value.into());`.
    pub fn new_supported_products<ValueType: Into<Vec<SupportedProductConfig>>>(mut self,
                                                                                value: ValueType)
                                                                                -> Self {
        self.new_supported_products = Some(value.into());
        self
    }
    /// Sets `release_label`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.release_label = Some(value.into());`.
    pub fn release_label<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.release_label = Some(value.into());
        self
    }
    /// Sets `repo_upgrade_on_boot`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.repo_upgrade_on_boot = Some(value.into());`.
    pub fn repo_upgrade_on_boot<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.repo_upgrade_on_boot = Some(value.into());
        self
    }
    /// Sets `scale_down_behavior`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.scale_down_behavior = Some(value.into());`.
    pub fn scale_down_behavior<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.scale_down_behavior = Some(value.into());
        self
    }
    /// Sets `security_configuration`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.security_configuration = Some(value.into());`.
    pub fn security_configuration<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.security_configuration = Some(value.into());
        self
    }
    /// Sets `service_role`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.service_role = Some(value.into());`.
    pub fn service_role<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.service_role = Some(value.into());
        self
    }
    /// Sets `steps`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.steps = Some(value.into());`.
    pub fn steps<ValueType: Into<Vec<StepConfig>>>(mut self, value: ValueType) -> Self {
        self.steps = Some(value.into());
        self
    }
    /// Sets `supported_products`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.supported_products = Some(value.into());`.
    pub fn supported_products<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.supported_products = Some(value.into());
        self
    }
    /// Sets `tags`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.tags = Some(value.into());`.
    pub fn tags<ValueType: Into<Vec<Tag>>>(mut self, value: ValueType) -> Self {
        self.tags = Some(value.into());
        self
    }
    /// Sets `visible_to_all_users`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `RunJobFlowInput.visible_to_all_users = Some(value.into());`.
    pub fn visible_to_all_users<ValueType: Into<bool>>(mut self, value: ValueType) -> Self {
        self.visible_to_all_users = Some(value.into());
        self
    }
    /// Returns a new instance of RunJobFlowInput with optional fields set to `None`.
    pub fn new<InstancesType: Into<JobFlowInstancesConfig>, NameType: Into<String>>
        (instances: InstancesType,
         name: NameType)
         -> RunJobFlowInput {
        RunJobFlowInput {
            instances: instances.into(),
            name: name.into(),
            ..Default::default()
        }
    }
}
#[doc="<p> The result of the <a>RunJobFlow</a> operation. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RunJobFlowOutput {
    #[doc="<p>An unique identifier for the job flow.</p>"]
    #[serde(rename="JobFlowId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub job_flow_id: Option<String>,
}
#[doc="<p>The type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScalingAction {
    #[doc="<p>Not available for instance groups. Instance groups use the market type specified for the group.</p>"]
    #[serde(rename="Market")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub market: Option<String>,
    #[doc="<p>The type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.</p>"]
    #[serde(rename="SimpleScalingPolicyConfiguration")]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,
}
impl ScalingAction {
    /// Sets `market`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingAction.market = Some(value.into());`.
    pub fn market<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.market = Some(value.into());
        self
    }
    /// Sets `simple_scaling_policy_configuration`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingAction.simple_scaling_policy_configuration = value.into();`.
    pub fn simple_scaling_policy_configuration<ValueType: Into<SimpleScalingPolicyConfiguration>>
        (mut self,
         value: ValueType)
         -> Self {
        self.simple_scaling_policy_configuration = value.into();
        self
    }
    /// Returns a new instance of ScalingAction with optional fields set to `None`.
    pub fn new<SimpleScalingPolicyConfigurationType: Into<SimpleScalingPolicyConfiguration>>
        (simple_scaling_policy_configuration: SimpleScalingPolicyConfigurationType)
         -> ScalingAction {
        ScalingAction {
            simple_scaling_policy_configuration: simple_scaling_policy_configuration.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activities triggered by automatic scaling rules will not cause an instance group to grow above or below these limits.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScalingConstraints {
    #[doc="<p>The upper boundary of EC2 instances in an instance group beyond which scaling activities are not allowed to grow. Scale-out activities will not add instances beyond this boundary.</p>"]
    #[serde(rename="MaxCapacity")]
    pub max_capacity: i64,
    #[doc="<p>The lower boundary of EC2 instances in an instance group below which scaling activities are not allowed to shrink. Scale-in activities will not terminate instances below this boundary.</p>"]
    #[serde(rename="MinCapacity")]
    pub min_capacity: i64,
}
impl ScalingConstraints {
    /// Sets `max_capacity`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingConstraints.max_capacity = value.into();`.
    pub fn max_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.max_capacity = value.into();
        self
    }
    /// Sets `min_capacity`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingConstraints.min_capacity = value.into();`.
    pub fn min_capacity<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.min_capacity = value.into();
        self
    }
    /// Returns a new instance of ScalingConstraints with optional fields set to `None`.
pub fn new<MaxCapacityType: Into<i64>, MinCapacityType: Into<i64>>(max_capacity: MaxCapacityType, min_capacity: MinCapacityType) -> ScalingConstraints{
        ScalingConstraints {
            max_capacity: max_capacity.into(),
            min_capacity: min_capacity.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>A scale-in or scale-out rule that defines scaling activity, including the CloudWatch metric alarm that triggers activity, how EC2 instances are added or removed, and the periodicity of adjustments. The automatic scaling policy for an instance group can comprise one or more automatic scaling rules.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScalingRule {
    #[doc="<p>The conditions that trigger an automatic scaling activity.</p>"]
    #[serde(rename="Action")]
    pub action: ScalingAction,
    #[doc="<p>A friendly, more verbose description of the automatic scaling rule.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>The name used to identify an automatic scaling rule. Rule names must be unique within a scaling policy.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The CloudWatch alarm definition that determines when automatic scaling activity is triggered.</p>"]
    #[serde(rename="Trigger")]
    pub trigger: ScalingTrigger,
}
impl ScalingRule {
    /// Sets `action`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingRule.action = value.into();`.
    pub fn action<ValueType: Into<ScalingAction>>(mut self, value: ValueType) -> Self {
        self.action = value.into();
        self
    }
    /// Sets `description`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingRule.description = Some(value.into());`.
    pub fn description<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingRule.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `trigger`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingRule.trigger = value.into();`.
    pub fn trigger<ValueType: Into<ScalingTrigger>>(mut self, value: ValueType) -> Self {
        self.trigger = value.into();
        self
    }
    /// Returns a new instance of ScalingRule with optional fields set to `None`.
    pub fn new<ActionType: Into<ScalingAction>,
               NameType: Into<String>,
               TriggerType: Into<ScalingTrigger>>
        (action: ActionType,
         name: NameType,
         trigger: TriggerType)
         -> ScalingRule {
        ScalingRule {
            action: action.into(),
            name: name.into(),
            trigger: trigger.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The conditions that trigger an automatic scaling activity.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScalingTrigger {
    #[doc="<p>The definition of a CloudWatch metric alarm. When the defined alarm conditions are met along with other trigger parameters, scaling activity begins.</p>"]
    #[serde(rename="CloudWatchAlarmDefinition")]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,
}
impl ScalingTrigger {
    /// Sets `cloud_watch_alarm_definition`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScalingTrigger.cloud_watch_alarm_definition = value.into();`.
pub fn cloud_watch_alarm_definition<ValueType: Into<CloudWatchAlarmDefinition>>(mut self, value: ValueType) -> Self{
        self.cloud_watch_alarm_definition = value.into();
        self
    }
    /// Returns a new instance of ScalingTrigger with optional fields set to `None`.
    pub fn new<CloudWatchAlarmDefinitionType: Into<CloudWatchAlarmDefinition>>
        (cloud_watch_alarm_definition: CloudWatchAlarmDefinitionType)
         -> ScalingTrigger {
        ScalingTrigger {
            cloud_watch_alarm_definition: cloud_watch_alarm_definition.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Configuration of the script to run during a bootstrap action.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScriptBootstrapActionConfig {
    #[doc="<p>A list of command line arguments to pass to the bootstrap action script.</p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<Vec<String>>,
    #[doc="<p>Location of the script to run during a bootstrap action. Can be either a location in Amazon S3 or on a local file system.</p>"]
    #[serde(rename="Path")]
    pub path: String,
}
impl ScriptBootstrapActionConfig {
    /// Sets `args`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScriptBootstrapActionConfig.args = Some(value.into());`.
    pub fn args<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.args = Some(value.into());
        self
    }
    /// Sets `path`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ScriptBootstrapActionConfig.path = value.into();`.
    pub fn path<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.path = value.into();
        self
    }
    /// Returns a new instance of ScriptBootstrapActionConfig with optional fields set to `None`.
    pub fn new<PathType: Into<String>>(path: PathType) -> ScriptBootstrapActionConfig {
        ScriptBootstrapActionConfig {
            path: path.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The creation date and time, and name, of a security configuration.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SecurityConfigurationSummary {
    #[doc="<p>The date and time the security configuration was created.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The name of the security configuration.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}
#[doc="<p> The input argument to the <a>TerminationProtection</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetTerminationProtectionInput {
    #[doc="<p> A list of strings that uniquely identify the clusters to protect. This identifier is returned by <a>RunJobFlow</a> and can also be obtained from <a>DescribeJobFlows</a> . </p>"]
    #[serde(rename="JobFlowIds")]
    pub job_flow_ids: Vec<String>,
    #[doc="<p>A Boolean that indicates whether to protect the cluster and prevent the Amazon EC2 instances in the cluster from shutting down due to API calls, user intervention, or job-flow error.</p>"]
    #[serde(rename="TerminationProtected")]
    pub termination_protected: bool,
}
impl SetTerminationProtectionInput {
    /// Sets `job_flow_ids`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SetTerminationProtectionInput.job_flow_ids = value.into();`.
    pub fn job_flow_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.job_flow_ids = value.into();
        self
    }
    /// Sets `termination_protected`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SetTerminationProtectionInput.termination_protected = value.into();`.
    pub fn termination_protected<ValueType: Into<bool>>(mut self, value: ValueType) -> Self {
        self.termination_protected = value.into();
        self
    }
    /// Returns a new instance of SetTerminationProtectionInput with optional fields set to `None`.
    pub fn new<JobFlowIdsType: Into<Vec<String>>, TerminationProtectedType: Into<bool>>
        (job_flow_ids: JobFlowIdsType,
         termination_protected: TerminationProtectedType)
         -> SetTerminationProtectionInput {
        SetTerminationProtectionInput {
            job_flow_ids: job_flow_ids.into(),
            termination_protected: termination_protected.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The input to the SetVisibleToAllUsers action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetVisibleToAllUsersInput {
    #[doc="<p>Identifiers of the job flows to receive the new visibility setting.</p>"]
    #[serde(rename="JobFlowIds")]
    pub job_flow_ids: Vec<String>,
    #[doc="<p>Whether the specified clusters are visible to all IAM users of the AWS account associated with the cluster. If this value is set to True, all IAM users of that AWS account can view and, if they have the proper IAM policy permissions set, manage the clusters. If it is set to False, only the IAM user that created a cluster can view and manage it.</p>"]
    #[serde(rename="VisibleToAllUsers")]
    pub visible_to_all_users: bool,
}
impl SetVisibleToAllUsersInput {
    /// Sets `job_flow_ids`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SetVisibleToAllUsersInput.job_flow_ids = value.into();`.
    pub fn job_flow_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.job_flow_ids = value.into();
        self
    }
    /// Sets `visible_to_all_users`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SetVisibleToAllUsersInput.visible_to_all_users = value.into();`.
    pub fn visible_to_all_users<ValueType: Into<bool>>(mut self, value: ValueType) -> Self {
        self.visible_to_all_users = value.into();
        self
    }
    /// Returns a new instance of SetVisibleToAllUsersInput with optional fields set to `None`.
    pub fn new<JobFlowIdsType: Into<Vec<String>>, VisibleToAllUsersType: Into<bool>>
        (job_flow_ids: JobFlowIdsType,
         visible_to_all_users: VisibleToAllUsersType)
         -> SetVisibleToAllUsersInput {
        SetVisibleToAllUsersInput {
            job_flow_ids: job_flow_ids.into(),
            visible_to_all_users: visible_to_all_users.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Policy for customizing shrink operations. Allows configuration of decommissioning timeout and targeted instance shrinking.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ShrinkPolicy {
    #[doc="<p>The desired timeout for decommissioning an instance. Overrides the default YARN decommissioning timeout.</p>"]
    #[serde(rename="DecommissionTimeout")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub decommission_timeout: Option<i64>,
    #[doc="<p>Custom policy for requesting termination protection or termination of specific instances when shrinking an instance group.</p>"]
    #[serde(rename="InstanceResizePolicy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_resize_policy: Option<InstanceResizePolicy>,
}
impl ShrinkPolicy {
    /// Sets `decommission_timeout`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ShrinkPolicy.decommission_timeout = Some(value.into());`.
    pub fn decommission_timeout<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.decommission_timeout = Some(value.into());
        self
    }
    /// Sets `instance_resize_policy`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `ShrinkPolicy.instance_resize_policy = Some(value.into());`.
    pub fn instance_resize_policy<ValueType: Into<InstanceResizePolicy>>(mut self,
                                                                         value: ValueType)
                                                                         -> Self {
        self.instance_resize_policy = Some(value.into());
        self
    }
    /// Returns a new instance of ShrinkPolicy with optional fields set to `None`.
    pub fn new() -> ShrinkPolicy {
        ShrinkPolicy { ..Default::default() }
    }
}
#[doc="<p>An automatic scaling configuration, which describes how the policy adds or removes instances, the cooldown period, and the number of EC2 instances that will be added each time the CloudWatch metric alarm condition is satisfied.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SimpleScalingPolicyConfiguration {
    #[doc="<p>The way in which EC2 instances are added (if <code>ScalingAdjustment</code> is a positive number) or terminated (if <code>ScalingAdjustment</code> is a negative number) each time the scaling activity is triggered. <code>CHANGE_IN_CAPACITY</code> is the default. <code>CHANGE_IN_CAPACITY</code> indicates that the EC2 instance count increments or decrements by <code>ScalingAdjustment</code>, which should be expressed as an integer. <code>PERCENT_CHANGE_IN_CAPACITY</code> indicates the instance count increments or decrements by the percentage specified by <code>ScalingAdjustment</code>, which should be expressed as a decimal. For example, 0.20 indicates an increase in 20% increments of cluster capacity. <code>EXACT_CAPACITY</code> indicates the scaling activity results in an instance group with the number of EC2 instances specified by <code>ScalingAdjustment</code>, which should be expressed as a positive integer.</p>"]
    #[serde(rename="AdjustmentType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub adjustment_type: Option<String>,
    #[doc="<p>The amount of time, in seconds, after a scaling activity completes before any further trigger-related scaling activities can start. The default value is 0.</p>"]
    #[serde(rename="CoolDown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cool_down: Option<i64>,
    #[doc="<p>The amount by which to scale in or scale out, based on the specified <code>AdjustmentType</code>. A positive value adds to the instance group's EC2 instance count while a negative number removes instances. If <code>AdjustmentType</code> is set to <code>EXACT_CAPACITY</code>, the number should only be a positive integer. If <code>AdjustmentType</code> is set to <code>PERCENT_CHANGE_IN_CAPACITY</code>, the value should express the percentage as a decimal. For example, -0.20 indicates a decrease in 20% increments of cluster capacity.</p>"]
    #[serde(rename="ScalingAdjustment")]
    pub scaling_adjustment: i64,
}
impl SimpleScalingPolicyConfiguration {
    /// Sets `adjustment_type`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SimpleScalingPolicyConfiguration.adjustment_type = Some(value.into());`.
    pub fn adjustment_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.adjustment_type = Some(value.into());
        self
    }
    /// Sets `cool_down`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SimpleScalingPolicyConfiguration.cool_down = Some(value.into());`.
    pub fn cool_down<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.cool_down = Some(value.into());
        self
    }
    /// Sets `scaling_adjustment`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SimpleScalingPolicyConfiguration.scaling_adjustment = value.into();`.
    pub fn scaling_adjustment<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.scaling_adjustment = value.into();
        self
    }
    /// Returns a new instance of SimpleScalingPolicyConfiguration with optional fields set to `None`.
    pub fn new<ScalingAdjustmentType: Into<i64>>(scaling_adjustment: ScalingAdjustmentType)
                                                 -> SimpleScalingPolicyConfiguration {
        SimpleScalingPolicyConfiguration {
            scaling_adjustment: scaling_adjustment.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>The launch specification for Spot instances in the instance fleet, which determines the defined duration and provisioning timeout behavior.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SpotProvisioningSpecification {
    #[doc="<p>The defined duration for Spot instances (also known as Spot blocks) in minutes. When specified, the Spot instance does not terminate before the defined duration expires, and defined duration pricing for Spot instances applies. Valid values are 60, 120, 180, 240, 300, or 360. The duration period starts as soon as a Spot instance receives its instance ID. At the end of the duration, Amazon EC2 marks the Spot instance for termination and provides a Spot instance termination notice, which gives the instance a two-minute warning before it terminates. </p>"]
    #[serde(rename="BlockDurationMinutes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub block_duration_minutes: Option<i64>,
    #[doc="<p>The action to take when <code>TargetSpotCapacity</code> has not been fulfilled when the <code>TimeoutDurationMinutes</code> has expired. Spot instances are not uprovisioned within the Spot provisioining timeout. Valid values are <code>TERMINATE_CLUSTER</code> and <code>SWITCH_TO_ON_DEMAND</code>. SWITCH_TO_ON_DEMAND specifies that if no Spot instances are available, On-Demand Instances should be provisioned to fulfill any remaining Spot capacity.</p>"]
    #[serde(rename="TimeoutAction")]
    pub timeout_action: String,
    #[doc="<p>The spot provisioning timeout period in minutes. If Spot instances are not provisioned within this time period, the <code>TimeOutAction</code> is taken. Minimum value is 5 and maximum value is 1440. The timeout applies only during initial provisioning, when the cluster is first created.</p>"]
    #[serde(rename="TimeoutDurationMinutes")]
    pub timeout_duration_minutes: i64,
}
impl SpotProvisioningSpecification {
    /// Sets `block_duration_minutes`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SpotProvisioningSpecification.block_duration_minutes = Some(value.into());`.
    pub fn block_duration_minutes<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.block_duration_minutes = Some(value.into());
        self
    }
    /// Sets `timeout_action`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SpotProvisioningSpecification.timeout_action = value.into();`.
    pub fn timeout_action<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.timeout_action = value.into();
        self
    }
    /// Sets `timeout_duration_minutes`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SpotProvisioningSpecification.timeout_duration_minutes = value.into();`.
    pub fn timeout_duration_minutes<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.timeout_duration_minutes = value.into();
        self
    }
    /// Returns a new instance of SpotProvisioningSpecification with optional fields set to `None`.
    pub fn new<TimeoutActionType: Into<String>, TimeoutDurationMinutesType: Into<i64>>
        (timeout_action: TimeoutActionType,
         timeout_duration_minutes: TimeoutDurationMinutesType)
         -> SpotProvisioningSpecification {
        SpotProvisioningSpecification {
            timeout_action: timeout_action.into(),
            timeout_duration_minutes: timeout_duration_minutes.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>This represents a step in a cluster.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Step {
    #[doc="<p>This specifies what action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE.</p>"]
    #[serde(rename="ActionOnFailure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action_on_failure: Option<String>,
    #[doc="<p>The Hadoop job configuration of the cluster step.</p>"]
    #[serde(rename="Config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<HadoopStepConfig>,
    #[doc="<p>The identifier of the cluster step.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The name of the cluster step.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The current execution status details of the cluster step.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<StepStatus>,
}
#[doc="<p>Specification of a cluster (job flow) step.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct StepConfig {
    #[doc="<p>The action to take if the step fails.</p>"]
    #[serde(rename="ActionOnFailure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action_on_failure: Option<String>,
    #[doc="<p>The JAR file used for the step.</p>"]
    #[serde(rename="HadoopJarStep")]
    pub hadoop_jar_step: HadoopJarStepConfig,
    #[doc="<p>The name of the step.</p>"]
    #[serde(rename="Name")]
    pub name: String,
}
impl StepConfig {
    /// Sets `action_on_failure`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `StepConfig.action_on_failure = Some(value.into());`.
    pub fn action_on_failure<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.action_on_failure = Some(value.into());
        self
    }
    /// Sets `hadoop_jar_step`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `StepConfig.hadoop_jar_step = value.into();`.
    pub fn hadoop_jar_step<ValueType: Into<HadoopJarStepConfig>>(mut self,
                                                                 value: ValueType)
                                                                 -> Self {
        self.hadoop_jar_step = value.into();
        self
    }
    /// Sets `name`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `StepConfig.name = value.into();`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = value.into();
        self
    }
    /// Returns a new instance of StepConfig with optional fields set to `None`.
    pub fn new<HadoopJarStepType: Into<HadoopJarStepConfig>, NameType: Into<String>>
        (hadoop_jar_step: HadoopJarStepType,
         name: NameType)
         -> StepConfig {
        StepConfig {
            hadoop_jar_step: hadoop_jar_step.into(),
            name: name.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>Combines the execution state and configuration of a step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StepDetail {
    #[doc="<p>The description of the step status.</p>"]
    #[serde(rename="ExecutionStatusDetail")]
    pub execution_status_detail: StepExecutionStatusDetail,
    #[doc="<p>The step configuration.</p>"]
    #[serde(rename="StepConfig")]
    pub step_config: StepConfig,
}
#[doc="<p>The execution state of a step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StepExecutionStatusDetail {
    #[doc="<p>The creation date and time of the step.</p>"]
    #[serde(rename="CreationDateTime")]
    pub creation_date_time: f64,
    #[doc="<p>The completion date and time of the step.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>A description of the step's current state.</p>"]
    #[serde(rename="LastStateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_state_change_reason: Option<String>,
    #[doc="<p>The start date and time of the step.</p>"]
    #[serde(rename="StartDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_date_time: Option<f64>,
    #[doc="<p>The state of the step.</p>"]
    #[serde(rename="State")]
    pub state: String,
}
#[doc="<p>The details of the step state change reason.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StepStateChangeReason {
    #[doc="<p>The programmable code for the state change reason. Note: Currently, the service provides no code for the state change.</p>"]
    #[serde(rename="Code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[doc="<p>The descriptive message for the state change reason.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}
#[doc="<p>The execution status details of the cluster step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StepStatus {
    #[doc="<p>The details for the step failure including reason, message, and log file path where the root cause was identified.</p>"]
    #[serde(rename="FailureDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failure_details: Option<FailureDetails>,
    #[doc="<p>The execution state of the cluster step.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[doc="<p>The reason for the step execution status change.</p>"]
    #[serde(rename="StateChangeReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state_change_reason: Option<StepStateChangeReason>,
    #[doc="<p>The timeline of the cluster step status over time.</p>"]
    #[serde(rename="Timeline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline: Option<StepTimeline>,
}
#[doc="<p>The summary of the cluster step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StepSummary {
    #[doc="<p>This specifies what action to take when the cluster step fails. Possible values are TERMINATE_CLUSTER, CANCEL_AND_WAIT, and CONTINUE.</p>"]
    #[serde(rename="ActionOnFailure")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action_on_failure: Option<String>,
    #[doc="<p>The Hadoop job configuration of the cluster step.</p>"]
    #[serde(rename="Config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<HadoopStepConfig>,
    #[doc="<p>The identifier of the cluster step.</p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[doc="<p>The name of the cluster step.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The current execution status details of the cluster step.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<StepStatus>,
}
#[doc="<p>The timeline of the cluster step lifecycle.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StepTimeline {
    #[doc="<p>The date and time when the cluster step was created.</p>"]
    #[serde(rename="CreationDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[doc="<p>The date and time when the cluster step execution completed or failed.</p>"]
    #[serde(rename="EndDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date_time: Option<f64>,
    #[doc="<p>The date and time when the cluster step execution started.</p>"]
    #[serde(rename="StartDateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_date_time: Option<f64>,
}
#[doc="<p>The list of supported product configurations which allow user-supplied arguments. EMR accepts these arguments and forwards them to the corresponding installation script as bootstrap action arguments.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SupportedProductConfig {
    #[doc="<p>The list of user-supplied arguments.</p>"]
    #[serde(rename="Args")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub args: Option<Vec<String>>,
    #[doc="<p>The name of the product configuration.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}
impl SupportedProductConfig {
    /// Sets `args`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SupportedProductConfig.args = Some(value.into());`.
    pub fn args<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.args = Some(value.into());
        self
    }
    /// Sets `name`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `SupportedProductConfig.name = Some(value.into());`.
    pub fn name<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Returns a new instance of SupportedProductConfig with optional fields set to `None`.
    pub fn new() -> SupportedProductConfig {
        SupportedProductConfig { ..Default::default() }
    }
}
#[doc="<p>A key/value pair containing user-defined metadata that you can associate with an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>A user-defined key, which is the minimum required information for a valid tag. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="<p>A user-defined value, which is optional in a tag. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}
impl Tag {
    /// Sets `key`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Tag.key = Some(value.into());`.
    pub fn key<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets `value`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `Tag.value = Some(value.into());`.
    pub fn value<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Returns a new instance of Tag with optional fields set to `None`.
    pub fn new() -> Tag {
        Tag { ..Default::default() }
    }
}
#[doc="<p> Input to the <a>TerminateJobFlows</a> operation. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TerminateJobFlowsInput {
    #[doc="<p>A list of job flows to be shutdown.</p>"]
    #[serde(rename="JobFlowIds")]
    pub job_flow_ids: Vec<String>,
}
impl TerminateJobFlowsInput {
    /// Sets `job_flow_ids`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `TerminateJobFlowsInput.job_flow_ids = value.into();`.
    pub fn job_flow_ids<ValueType: Into<Vec<String>>>(mut self, value: ValueType) -> Self {
        self.job_flow_ids = value.into();
        self
    }
    /// Returns a new instance of TerminateJobFlowsInput with optional fields set to `None`.
    pub fn new<JobFlowIdsType: Into<Vec<String>>>(job_flow_ids: JobFlowIdsType)
                                                  -> TerminateJobFlowsInput {
        TerminateJobFlowsInput {
            job_flow_ids: job_flow_ids.into(),
            ..Default::default()
        }
    }
}
#[doc="<p>EBS volume specifications such as volume type, IOPS, and size (GiB) that will be requested for the EBS volume attached to an EC2 instance in the cluster.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct VolumeSpecification {
    #[doc="<p>The number of I/O operations per second (IOPS) that the volume supports.</p>"]
    #[serde(rename="Iops")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub iops: Option<i64>,
    #[doc="<p>The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume type is EBS-optimized, the minimum value is 10.</p>"]
    #[serde(rename="SizeInGB")]
    pub size_in_gb: i64,
    #[doc="<p>The volume type. Volume types supported are gp2, io1, standard.</p>"]
    #[serde(rename="VolumeType")]
    pub volume_type: String,
}
impl VolumeSpecification {
    /// Sets `iops`, wrapping it with `Some()` and invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `VolumeSpecification.iops = Some(value.into());`.
    pub fn iops<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.iops = Some(value.into());
        self
    }
    /// Sets `size_in_gb`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `VolumeSpecification.size_in_gb = value.into();`.
    pub fn size_in_gb<ValueType: Into<i64>>(mut self, value: ValueType) -> Self {
        self.size_in_gb = value.into();
        self
    }
    /// Sets `volume_type`, invoking `.into()` to convert to the required type.
    ///
    /// Equivalent to `VolumeSpecification.volume_type = value.into();`.
    pub fn volume_type<ValueType: Into<String>>(mut self, value: ValueType) -> Self {
        self.volume_type = value.into();
        self
    }
    /// Returns a new instance of VolumeSpecification with optional fields set to `None`.
    pub fn new<SizeInGBType: Into<i64>, VolumeTypeType: Into<String>>(size_in_gb: SizeInGBType,
                                                                      volume_type: VolumeTypeType)
                                                                      -> VolumeSpecification {
        VolumeSpecification {
            size_in_gb: size_in_gb.into(),
            volume_type: volume_type.into(),
            ..Default::default()
        }
    }
}
/// Errors returned by AddInstanceFleet
#[derive(Debug, PartialEq)]
pub enum AddInstanceFleetError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddInstanceFleetError {
    pub fn from_body(body: &str) -> AddInstanceFleetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        AddInstanceFleetError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AddInstanceFleetError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddInstanceFleetError::Validation(error_message.to_string())
                    }
                    _ => AddInstanceFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddInstanceFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddInstanceFleetError {
    fn from(err: serde_json::error::Error) -> AddInstanceFleetError {
        AddInstanceFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddInstanceFleetError {
    fn from(err: CredentialsError) -> AddInstanceFleetError {
        AddInstanceFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddInstanceFleetError {
    fn from(err: HttpDispatchError) -> AddInstanceFleetError {
        AddInstanceFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddInstanceFleetError {
    fn from(err: io::Error) -> AddInstanceFleetError {
        AddInstanceFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddInstanceFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddInstanceFleetError {
    fn description(&self) -> &str {
        match *self {
            AddInstanceFleetError::InternalServer(ref cause) => cause,
            AddInstanceFleetError::InvalidRequest(ref cause) => cause,
            AddInstanceFleetError::Validation(ref cause) => cause,
            AddInstanceFleetError::Credentials(ref err) => err.description(),
            AddInstanceFleetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddInstanceFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddInstanceGroups
#[derive(Debug, PartialEq)]
pub enum AddInstanceGroupsError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddInstanceGroupsError {
    pub fn from_body(body: &str) -> AddInstanceGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        AddInstanceGroupsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddInstanceGroupsError::Validation(error_message.to_string())
                    }
                    _ => AddInstanceGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddInstanceGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddInstanceGroupsError {
    fn from(err: serde_json::error::Error) -> AddInstanceGroupsError {
        AddInstanceGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddInstanceGroupsError {
    fn from(err: CredentialsError) -> AddInstanceGroupsError {
        AddInstanceGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddInstanceGroupsError {
    fn from(err: HttpDispatchError) -> AddInstanceGroupsError {
        AddInstanceGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddInstanceGroupsError {
    fn from(err: io::Error) -> AddInstanceGroupsError {
        AddInstanceGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddInstanceGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddInstanceGroupsError {
    fn description(&self) -> &str {
        match *self {
            AddInstanceGroupsError::InternalServerError(ref cause) => cause,
            AddInstanceGroupsError::Validation(ref cause) => cause,
            AddInstanceGroupsError::Credentials(ref err) => err.description(),
            AddInstanceGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddInstanceGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddJobFlowSteps
#[derive(Debug, PartialEq)]
pub enum AddJobFlowStepsError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddJobFlowStepsError {
    pub fn from_body(body: &str) -> AddJobFlowStepsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        AddJobFlowStepsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddJobFlowStepsError::Validation(error_message.to_string())
                    }
                    _ => AddJobFlowStepsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddJobFlowStepsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddJobFlowStepsError {
    fn from(err: serde_json::error::Error) -> AddJobFlowStepsError {
        AddJobFlowStepsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddJobFlowStepsError {
    fn from(err: CredentialsError) -> AddJobFlowStepsError {
        AddJobFlowStepsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddJobFlowStepsError {
    fn from(err: HttpDispatchError) -> AddJobFlowStepsError {
        AddJobFlowStepsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddJobFlowStepsError {
    fn from(err: io::Error) -> AddJobFlowStepsError {
        AddJobFlowStepsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddJobFlowStepsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddJobFlowStepsError {
    fn description(&self) -> &str {
        match *self {
            AddJobFlowStepsError::InternalServerError(ref cause) => cause,
            AddJobFlowStepsError::Validation(ref cause) => cause,
            AddJobFlowStepsError::Credentials(ref err) => err.description(),
            AddJobFlowStepsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddJobFlowStepsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddTagsError {
    pub fn from_body(body: &str) -> AddTagsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        AddTagsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AddTagsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => AddTagsError::Validation(error_message.to_string()),
                    _ => AddTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsError {
    fn from(err: serde_json::error::Error) -> AddTagsError {
        AddTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsError {
    fn from(err: CredentialsError) -> AddTagsError {
        AddTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsError {
    fn from(err: HttpDispatchError) -> AddTagsError {
        AddTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsError {
    fn from(err: io::Error) -> AddTagsError {
        AddTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsError {
    fn description(&self) -> &str {
        match *self {
            AddTagsError::InternalServer(ref cause) => cause,
            AddTagsError::InvalidRequest(ref cause) => cause,
            AddTagsError::Validation(ref cause) => cause,
            AddTagsError::Credentials(ref err) => err.description(),
            AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelSteps
#[derive(Debug, PartialEq)]
pub enum CancelStepsError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CancelStepsError {
    pub fn from_body(body: &str) -> CancelStepsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        CancelStepsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CancelStepsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelStepsError::Validation(error_message.to_string())
                    }
                    _ => CancelStepsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelStepsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelStepsError {
    fn from(err: serde_json::error::Error) -> CancelStepsError {
        CancelStepsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelStepsError {
    fn from(err: CredentialsError) -> CancelStepsError {
        CancelStepsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelStepsError {
    fn from(err: HttpDispatchError) -> CancelStepsError {
        CancelStepsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelStepsError {
    fn from(err: io::Error) -> CancelStepsError {
        CancelStepsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelStepsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelStepsError {
    fn description(&self) -> &str {
        match *self {
            CancelStepsError::InternalServerError(ref cause) => cause,
            CancelStepsError::InvalidRequest(ref cause) => cause,
            CancelStepsError::Validation(ref cause) => cause,
            CancelStepsError::Credentials(ref err) => err.description(),
            CancelStepsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelStepsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateSecurityConfigurationError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateSecurityConfigurationError {
    pub fn from_body(body: &str) -> CreateSecurityConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => CreateSecurityConfigurationError::InternalServer(String::from(error_message)),
                    "InvalidRequestException" => CreateSecurityConfigurationError::InvalidRequest(String::from(error_message)),
                    "ValidationException" => {
                        CreateSecurityConfigurationError::Validation(error_message.to_string())
                    }
                    _ => CreateSecurityConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSecurityConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSecurityConfigurationError {
    fn from(err: serde_json::error::Error) -> CreateSecurityConfigurationError {
        CreateSecurityConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSecurityConfigurationError {
    fn from(err: CredentialsError) -> CreateSecurityConfigurationError {
        CreateSecurityConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSecurityConfigurationError {
    fn from(err: HttpDispatchError) -> CreateSecurityConfigurationError {
        CreateSecurityConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSecurityConfigurationError {
    fn from(err: io::Error) -> CreateSecurityConfigurationError {
        CreateSecurityConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateSecurityConfigurationError::InternalServer(ref cause) => cause,
            CreateSecurityConfigurationError::InvalidRequest(ref cause) => cause,
            CreateSecurityConfigurationError::Validation(ref cause) => cause,
            CreateSecurityConfigurationError::Credentials(ref err) => err.description(),
            CreateSecurityConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSecurityConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteSecurityConfigurationError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteSecurityConfigurationError {
    pub fn from_body(body: &str) -> DeleteSecurityConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => DeleteSecurityConfigurationError::InternalServer(String::from(error_message)),
                    "InvalidRequestException" => DeleteSecurityConfigurationError::InvalidRequest(String::from(error_message)),
                    "ValidationException" => {
                        DeleteSecurityConfigurationError::Validation(error_message.to_string())
                    }
                    _ => DeleteSecurityConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSecurityConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSecurityConfigurationError {
    fn from(err: serde_json::error::Error) -> DeleteSecurityConfigurationError {
        DeleteSecurityConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSecurityConfigurationError {
    fn from(err: CredentialsError) -> DeleteSecurityConfigurationError {
        DeleteSecurityConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSecurityConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteSecurityConfigurationError {
        DeleteSecurityConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSecurityConfigurationError {
    fn from(err: io::Error) -> DeleteSecurityConfigurationError {
        DeleteSecurityConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteSecurityConfigurationError::InternalServer(ref cause) => cause,
            DeleteSecurityConfigurationError::InvalidRequest(ref cause) => cause,
            DeleteSecurityConfigurationError::Validation(ref cause) => cause,
            DeleteSecurityConfigurationError::Credentials(ref err) => err.description(),
            DeleteSecurityConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSecurityConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeClusterError {
    pub fn from_body(body: &str) -> DescribeClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        DescribeClusterError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeClusterError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeClusterError::Validation(error_message.to_string())
                    }
                    _ => DescribeClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeClusterError {
    fn from(err: serde_json::error::Error) -> DescribeClusterError {
        DescribeClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeClusterError {
    fn from(err: CredentialsError) -> DescribeClusterError {
        DescribeClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterError {
    fn from(err: HttpDispatchError) -> DescribeClusterError {
        DescribeClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterError {
    fn from(err: io::Error) -> DescribeClusterError {
        DescribeClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterError::InternalServer(ref cause) => cause,
            DescribeClusterError::InvalidRequest(ref cause) => cause,
            DescribeClusterError::Validation(ref cause) => cause,
            DescribeClusterError::Credentials(ref err) => err.description(),
            DescribeClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJobFlows
#[derive(Debug, PartialEq)]
pub enum DescribeJobFlowsError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeJobFlowsError {
    pub fn from_body(body: &str) -> DescribeJobFlowsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeJobFlowsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobFlowsError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobFlowsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobFlowsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobFlowsError {
    fn from(err: serde_json::error::Error) -> DescribeJobFlowsError {
        DescribeJobFlowsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobFlowsError {
    fn from(err: CredentialsError) -> DescribeJobFlowsError {
        DescribeJobFlowsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobFlowsError {
    fn from(err: HttpDispatchError) -> DescribeJobFlowsError {
        DescribeJobFlowsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobFlowsError {
    fn from(err: io::Error) -> DescribeJobFlowsError {
        DescribeJobFlowsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobFlowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobFlowsError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobFlowsError::InternalServerError(ref cause) => cause,
            DescribeJobFlowsError::Validation(ref cause) => cause,
            DescribeJobFlowsError::Credentials(ref err) => err.description(),
            DescribeJobFlowsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeJobFlowsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeSecurityConfigurationError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeSecurityConfigurationError {
    pub fn from_body(body: &str) -> DescribeSecurityConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => DescribeSecurityConfigurationError::InternalServer(String::from(error_message)),
                    "InvalidRequestException" => DescribeSecurityConfigurationError::InvalidRequest(String::from(error_message)),
                    "ValidationException" => {
                        DescribeSecurityConfigurationError::Validation(error_message.to_string())
                    }
                    _ => DescribeSecurityConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSecurityConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSecurityConfigurationError {
    fn from(err: serde_json::error::Error) -> DescribeSecurityConfigurationError {
        DescribeSecurityConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSecurityConfigurationError {
    fn from(err: CredentialsError) -> DescribeSecurityConfigurationError {
        DescribeSecurityConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSecurityConfigurationError {
    fn from(err: HttpDispatchError) -> DescribeSecurityConfigurationError {
        DescribeSecurityConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSecurityConfigurationError {
    fn from(err: io::Error) -> DescribeSecurityConfigurationError {
        DescribeSecurityConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DescribeSecurityConfigurationError::InternalServer(ref cause) => cause,
            DescribeSecurityConfigurationError::InvalidRequest(ref cause) => cause,
            DescribeSecurityConfigurationError::Validation(ref cause) => cause,
            DescribeSecurityConfigurationError::Credentials(ref err) => err.description(),
            DescribeSecurityConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSecurityConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStep
#[derive(Debug, PartialEq)]
pub enum DescribeStepError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStepError {
    pub fn from_body(body: &str) -> DescribeStepError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        DescribeStepError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeStepError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStepError::Validation(error_message.to_string())
                    }
                    _ => DescribeStepError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStepError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStepError {
    fn from(err: serde_json::error::Error) -> DescribeStepError {
        DescribeStepError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStepError {
    fn from(err: CredentialsError) -> DescribeStepError {
        DescribeStepError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStepError {
    fn from(err: HttpDispatchError) -> DescribeStepError {
        DescribeStepError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStepError {
    fn from(err: io::Error) -> DescribeStepError {
        DescribeStepError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStepError {
    fn description(&self) -> &str {
        match *self {
            DescribeStepError::InternalServer(ref cause) => cause,
            DescribeStepError::InvalidRequest(ref cause) => cause,
            DescribeStepError::Validation(ref cause) => cause,
            DescribeStepError::Credentials(ref err) => err.description(),
            DescribeStepError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStepError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBootstrapActions
#[derive(Debug, PartialEq)]
pub enum ListBootstrapActionsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBootstrapActionsError {
    pub fn from_body(body: &str) -> ListBootstrapActionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListBootstrapActionsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListBootstrapActionsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBootstrapActionsError::Validation(error_message.to_string())
                    }
                    _ => ListBootstrapActionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBootstrapActionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBootstrapActionsError {
    fn from(err: serde_json::error::Error) -> ListBootstrapActionsError {
        ListBootstrapActionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBootstrapActionsError {
    fn from(err: CredentialsError) -> ListBootstrapActionsError {
        ListBootstrapActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBootstrapActionsError {
    fn from(err: HttpDispatchError) -> ListBootstrapActionsError {
        ListBootstrapActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBootstrapActionsError {
    fn from(err: io::Error) -> ListBootstrapActionsError {
        ListBootstrapActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBootstrapActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBootstrapActionsError {
    fn description(&self) -> &str {
        match *self {
            ListBootstrapActionsError::InternalServer(ref cause) => cause,
            ListBootstrapActionsError::InvalidRequest(ref cause) => cause,
            ListBootstrapActionsError::Validation(ref cause) => cause,
            ListBootstrapActionsError::Credentials(ref err) => err.description(),
            ListBootstrapActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBootstrapActionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListClustersError {
    pub fn from_body(body: &str) -> ListClustersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListClustersError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListClustersError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListClustersError::Validation(error_message.to_string())
                    }
                    _ => ListClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListClustersError {
    fn from(err: serde_json::error::Error) -> ListClustersError {
        ListClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListClustersError {
    fn from(err: CredentialsError) -> ListClustersError {
        ListClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListClustersError {
    fn from(err: HttpDispatchError) -> ListClustersError {
        ListClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListClustersError {
    fn from(err: io::Error) -> ListClustersError {
        ListClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClustersError {
    fn description(&self) -> &str {
        match *self {
            ListClustersError::InternalServer(ref cause) => cause,
            ListClustersError::InvalidRequest(ref cause) => cause,
            ListClustersError::Validation(ref cause) => cause,
            ListClustersError::Credentials(ref err) => err.description(),
            ListClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInstanceFleets
#[derive(Debug, PartialEq)]
pub enum ListInstanceFleetsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListInstanceFleetsError {
    pub fn from_body(body: &str) -> ListInstanceFleetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListInstanceFleetsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListInstanceFleetsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListInstanceFleetsError::Validation(error_message.to_string())
                    }
                    _ => ListInstanceFleetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListInstanceFleetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListInstanceFleetsError {
    fn from(err: serde_json::error::Error) -> ListInstanceFleetsError {
        ListInstanceFleetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInstanceFleetsError {
    fn from(err: CredentialsError) -> ListInstanceFleetsError {
        ListInstanceFleetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInstanceFleetsError {
    fn from(err: HttpDispatchError) -> ListInstanceFleetsError {
        ListInstanceFleetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInstanceFleetsError {
    fn from(err: io::Error) -> ListInstanceFleetsError {
        ListInstanceFleetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInstanceFleetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInstanceFleetsError {
    fn description(&self) -> &str {
        match *self {
            ListInstanceFleetsError::InternalServer(ref cause) => cause,
            ListInstanceFleetsError::InvalidRequest(ref cause) => cause,
            ListInstanceFleetsError::Validation(ref cause) => cause,
            ListInstanceFleetsError::Credentials(ref err) => err.description(),
            ListInstanceFleetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListInstanceFleetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInstanceGroups
#[derive(Debug, PartialEq)]
pub enum ListInstanceGroupsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListInstanceGroupsError {
    pub fn from_body(body: &str) -> ListInstanceGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListInstanceGroupsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListInstanceGroupsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListInstanceGroupsError::Validation(error_message.to_string())
                    }
                    _ => ListInstanceGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListInstanceGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListInstanceGroupsError {
    fn from(err: serde_json::error::Error) -> ListInstanceGroupsError {
        ListInstanceGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInstanceGroupsError {
    fn from(err: CredentialsError) -> ListInstanceGroupsError {
        ListInstanceGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInstanceGroupsError {
    fn from(err: HttpDispatchError) -> ListInstanceGroupsError {
        ListInstanceGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInstanceGroupsError {
    fn from(err: io::Error) -> ListInstanceGroupsError {
        ListInstanceGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInstanceGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInstanceGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListInstanceGroupsError::InternalServer(ref cause) => cause,
            ListInstanceGroupsError::InvalidRequest(ref cause) => cause,
            ListInstanceGroupsError::Validation(ref cause) => cause,
            ListInstanceGroupsError::Credentials(ref err) => err.description(),
            ListInstanceGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListInstanceGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInstances
#[derive(Debug, PartialEq)]
pub enum ListInstancesError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListInstancesError {
    pub fn from_body(body: &str) -> ListInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListInstancesError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListInstancesError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListInstancesError::Validation(error_message.to_string())
                    }
                    _ => ListInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListInstancesError {
    fn from(err: serde_json::error::Error) -> ListInstancesError {
        ListInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInstancesError {
    fn from(err: CredentialsError) -> ListInstancesError {
        ListInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInstancesError {
    fn from(err: HttpDispatchError) -> ListInstancesError {
        ListInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInstancesError {
    fn from(err: io::Error) -> ListInstancesError {
        ListInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListInstancesError::InternalServer(ref cause) => cause,
            ListInstancesError::InvalidRequest(ref cause) => cause,
            ListInstancesError::Validation(ref cause) => cause,
            ListInstancesError::Credentials(ref err) => err.description(),
            ListInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSecurityConfigurations
#[derive(Debug, PartialEq)]
pub enum ListSecurityConfigurationsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListSecurityConfigurationsError {
    pub fn from_body(body: &str) -> ListSecurityConfigurationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListSecurityConfigurationsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListSecurityConfigurationsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListSecurityConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => ListSecurityConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSecurityConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSecurityConfigurationsError {
    fn from(err: serde_json::error::Error) -> ListSecurityConfigurationsError {
        ListSecurityConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSecurityConfigurationsError {
    fn from(err: CredentialsError) -> ListSecurityConfigurationsError {
        ListSecurityConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSecurityConfigurationsError {
    fn from(err: HttpDispatchError) -> ListSecurityConfigurationsError {
        ListSecurityConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSecurityConfigurationsError {
    fn from(err: io::Error) -> ListSecurityConfigurationsError {
        ListSecurityConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSecurityConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSecurityConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListSecurityConfigurationsError::InternalServer(ref cause) => cause,
            ListSecurityConfigurationsError::InvalidRequest(ref cause) => cause,
            ListSecurityConfigurationsError::Validation(ref cause) => cause,
            ListSecurityConfigurationsError::Credentials(ref err) => err.description(),
            ListSecurityConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSecurityConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSteps
#[derive(Debug, PartialEq)]
pub enum ListStepsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListStepsError {
    pub fn from_body(body: &str) -> ListStepsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ListStepsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListStepsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => ListStepsError::Validation(error_message.to_string()),
                    _ => ListStepsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListStepsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListStepsError {
    fn from(err: serde_json::error::Error) -> ListStepsError {
        ListStepsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListStepsError {
    fn from(err: CredentialsError) -> ListStepsError {
        ListStepsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStepsError {
    fn from(err: HttpDispatchError) -> ListStepsError {
        ListStepsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStepsError {
    fn from(err: io::Error) -> ListStepsError {
        ListStepsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStepsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStepsError {
    fn description(&self) -> &str {
        match *self {
            ListStepsError::InternalServer(ref cause) => cause,
            ListStepsError::InvalidRequest(ref cause) => cause,
            ListStepsError::Validation(ref cause) => cause,
            ListStepsError::Credentials(ref err) => err.description(),
            ListStepsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStepsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyInstanceFleet
#[derive(Debug, PartialEq)]
pub enum ModifyInstanceFleetError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyInstanceFleetError {
    pub fn from_body(body: &str) -> ModifyInstanceFleetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        ModifyInstanceFleetError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ModifyInstanceFleetError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ModifyInstanceFleetError::Validation(error_message.to_string())
                    }
                    _ => ModifyInstanceFleetError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyInstanceFleetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyInstanceFleetError {
    fn from(err: serde_json::error::Error) -> ModifyInstanceFleetError {
        ModifyInstanceFleetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyInstanceFleetError {
    fn from(err: CredentialsError) -> ModifyInstanceFleetError {
        ModifyInstanceFleetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyInstanceFleetError {
    fn from(err: HttpDispatchError) -> ModifyInstanceFleetError {
        ModifyInstanceFleetError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyInstanceFleetError {
    fn from(err: io::Error) -> ModifyInstanceFleetError {
        ModifyInstanceFleetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyInstanceFleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyInstanceFleetError {
    fn description(&self) -> &str {
        match *self {
            ModifyInstanceFleetError::InternalServer(ref cause) => cause,
            ModifyInstanceFleetError::InvalidRequest(ref cause) => cause,
            ModifyInstanceFleetError::Validation(ref cause) => cause,
            ModifyInstanceFleetError::Credentials(ref err) => err.description(),
            ModifyInstanceFleetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyInstanceFleetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyInstanceGroups
#[derive(Debug, PartialEq)]
pub enum ModifyInstanceGroupsError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyInstanceGroupsError {
    pub fn from_body(body: &str) -> ModifyInstanceGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ModifyInstanceGroupsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ModifyInstanceGroupsError::Validation(error_message.to_string())
                    }
                    _ => ModifyInstanceGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyInstanceGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyInstanceGroupsError {
    fn from(err: serde_json::error::Error) -> ModifyInstanceGroupsError {
        ModifyInstanceGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyInstanceGroupsError {
    fn from(err: CredentialsError) -> ModifyInstanceGroupsError {
        ModifyInstanceGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyInstanceGroupsError {
    fn from(err: HttpDispatchError) -> ModifyInstanceGroupsError {
        ModifyInstanceGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyInstanceGroupsError {
    fn from(err: io::Error) -> ModifyInstanceGroupsError {
        ModifyInstanceGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyInstanceGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyInstanceGroupsError {
    fn description(&self) -> &str {
        match *self {
            ModifyInstanceGroupsError::InternalServerError(ref cause) => cause,
            ModifyInstanceGroupsError::Validation(ref cause) => cause,
            ModifyInstanceGroupsError::Credentials(ref err) => err.description(),
            ModifyInstanceGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyInstanceGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAutoScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutAutoScalingPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutAutoScalingPolicyError {
    pub fn from_body(body: &str) -> PutAutoScalingPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        PutAutoScalingPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutAutoScalingPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutAutoScalingPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutAutoScalingPolicyError {
    fn from(err: serde_json::error::Error) -> PutAutoScalingPolicyError {
        PutAutoScalingPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutAutoScalingPolicyError {
    fn from(err: CredentialsError) -> PutAutoScalingPolicyError {
        PutAutoScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutAutoScalingPolicyError {
    fn from(err: HttpDispatchError) -> PutAutoScalingPolicyError {
        PutAutoScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutAutoScalingPolicyError {
    fn from(err: io::Error) -> PutAutoScalingPolicyError {
        PutAutoScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutAutoScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAutoScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutAutoScalingPolicyError::Validation(ref cause) => cause,
            PutAutoScalingPolicyError::Credentials(ref err) => err.description(),
            PutAutoScalingPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutAutoScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAutoScalingPolicy
#[derive(Debug, PartialEq)]
pub enum RemoveAutoScalingPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RemoveAutoScalingPolicyError {
    pub fn from_body(body: &str) -> RemoveAutoScalingPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        RemoveAutoScalingPolicyError::Validation(error_message.to_string())
                    }
                    _ => RemoveAutoScalingPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveAutoScalingPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveAutoScalingPolicyError {
    fn from(err: serde_json::error::Error) -> RemoveAutoScalingPolicyError {
        RemoveAutoScalingPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveAutoScalingPolicyError {
    fn from(err: CredentialsError) -> RemoveAutoScalingPolicyError {
        RemoveAutoScalingPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveAutoScalingPolicyError {
    fn from(err: HttpDispatchError) -> RemoveAutoScalingPolicyError {
        RemoveAutoScalingPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveAutoScalingPolicyError {
    fn from(err: io::Error) -> RemoveAutoScalingPolicyError {
        RemoveAutoScalingPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveAutoScalingPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAutoScalingPolicyError {
    fn description(&self) -> &str {
        match *self {
            RemoveAutoScalingPolicyError::Validation(ref cause) => cause,
            RemoveAutoScalingPolicyError::Credentials(ref err) => err.description(),
            RemoveAutoScalingPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveAutoScalingPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    ///<p>This exception occurs when there is an internal failure in the EMR service.</p>
    InternalServer(String),
    ///<p>This exception occurs when there is something wrong with user input.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RemoveTagsError {
    pub fn from_body(body: &str) -> RemoveTagsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerException" => {
                        RemoveTagsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RemoveTagsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => RemoveTagsError::Validation(error_message.to_string()),
                    _ => RemoveTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsError {
    fn from(err: serde_json::error::Error) -> RemoveTagsError {
        RemoveTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsError {
    fn from(err: CredentialsError) -> RemoveTagsError {
        RemoveTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsError {
    fn from(err: HttpDispatchError) -> RemoveTagsError {
        RemoveTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsError {
    fn from(err: io::Error) -> RemoveTagsError {
        RemoveTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsError::InternalServer(ref cause) => cause,
            RemoveTagsError::InvalidRequest(ref cause) => cause,
            RemoveTagsError::Validation(ref cause) => cause,
            RemoveTagsError::Credentials(ref err) => err.description(),
            RemoveTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RunJobFlow
#[derive(Debug, PartialEq)]
pub enum RunJobFlowError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RunJobFlowError {
    pub fn from_body(body: &str) -> RunJobFlowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        RunJobFlowError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => RunJobFlowError::Validation(error_message.to_string()),
                    _ => RunJobFlowError::Unknown(String::from(body)),
                }
            }
            Err(_) => RunJobFlowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RunJobFlowError {
    fn from(err: serde_json::error::Error) -> RunJobFlowError {
        RunJobFlowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RunJobFlowError {
    fn from(err: CredentialsError) -> RunJobFlowError {
        RunJobFlowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RunJobFlowError {
    fn from(err: HttpDispatchError) -> RunJobFlowError {
        RunJobFlowError::HttpDispatch(err)
    }
}
impl From<io::Error> for RunJobFlowError {
    fn from(err: io::Error) -> RunJobFlowError {
        RunJobFlowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RunJobFlowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RunJobFlowError {
    fn description(&self) -> &str {
        match *self {
            RunJobFlowError::InternalServerError(ref cause) => cause,
            RunJobFlowError::Validation(ref cause) => cause,
            RunJobFlowError::Credentials(ref err) => err.description(),
            RunJobFlowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RunJobFlowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetTerminationProtection
#[derive(Debug, PartialEq)]
pub enum SetTerminationProtectionError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetTerminationProtectionError {
    pub fn from_body(body: &str) -> SetTerminationProtectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => SetTerminationProtectionError::InternalServerError(String::from(error_message)),
                    "ValidationException" => {
                        SetTerminationProtectionError::Validation(error_message.to_string())
                    }
                    _ => SetTerminationProtectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetTerminationProtectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetTerminationProtectionError {
    fn from(err: serde_json::error::Error) -> SetTerminationProtectionError {
        SetTerminationProtectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetTerminationProtectionError {
    fn from(err: CredentialsError) -> SetTerminationProtectionError {
        SetTerminationProtectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetTerminationProtectionError {
    fn from(err: HttpDispatchError) -> SetTerminationProtectionError {
        SetTerminationProtectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetTerminationProtectionError {
    fn from(err: io::Error) -> SetTerminationProtectionError {
        SetTerminationProtectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetTerminationProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetTerminationProtectionError {
    fn description(&self) -> &str {
        match *self {
            SetTerminationProtectionError::InternalServerError(ref cause) => cause,
            SetTerminationProtectionError::Validation(ref cause) => cause,
            SetTerminationProtectionError::Credentials(ref err) => err.description(),
            SetTerminationProtectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetTerminationProtectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetVisibleToAllUsers
#[derive(Debug, PartialEq)]
pub enum SetVisibleToAllUsersError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetVisibleToAllUsersError {
    pub fn from_body(body: &str) -> SetVisibleToAllUsersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        SetVisibleToAllUsersError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetVisibleToAllUsersError::Validation(error_message.to_string())
                    }
                    _ => SetVisibleToAllUsersError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetVisibleToAllUsersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetVisibleToAllUsersError {
    fn from(err: serde_json::error::Error) -> SetVisibleToAllUsersError {
        SetVisibleToAllUsersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetVisibleToAllUsersError {
    fn from(err: CredentialsError) -> SetVisibleToAllUsersError {
        SetVisibleToAllUsersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetVisibleToAllUsersError {
    fn from(err: HttpDispatchError) -> SetVisibleToAllUsersError {
        SetVisibleToAllUsersError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetVisibleToAllUsersError {
    fn from(err: io::Error) -> SetVisibleToAllUsersError {
        SetVisibleToAllUsersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetVisibleToAllUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetVisibleToAllUsersError {
    fn description(&self) -> &str {
        match *self {
            SetVisibleToAllUsersError::InternalServerError(ref cause) => cause,
            SetVisibleToAllUsersError::Validation(ref cause) => cause,
            SetVisibleToAllUsersError::Credentials(ref err) => err.description(),
            SetVisibleToAllUsersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetVisibleToAllUsersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateJobFlows
#[derive(Debug, PartialEq)]
pub enum TerminateJobFlowsError {
    ///<p>Indicates that an error occurred while processing the request and that the request was not completed.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TerminateJobFlowsError {
    pub fn from_body(body: &str) -> TerminateJobFlowsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        TerminateJobFlowsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        TerminateJobFlowsError::Validation(error_message.to_string())
                    }
                    _ => TerminateJobFlowsError::Unknown(String::from(body)),
                }
            }
            Err(_) => TerminateJobFlowsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TerminateJobFlowsError {
    fn from(err: serde_json::error::Error) -> TerminateJobFlowsError {
        TerminateJobFlowsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateJobFlowsError {
    fn from(err: CredentialsError) -> TerminateJobFlowsError {
        TerminateJobFlowsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateJobFlowsError {
    fn from(err: HttpDispatchError) -> TerminateJobFlowsError {
        TerminateJobFlowsError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateJobFlowsError {
    fn from(err: io::Error) -> TerminateJobFlowsError {
        TerminateJobFlowsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateJobFlowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateJobFlowsError {
    fn description(&self) -> &str {
        match *self {
            TerminateJobFlowsError::InternalServerError(ref cause) => cause,
            TerminateJobFlowsError::Validation(ref cause) => cause,
            TerminateJobFlowsError::Credentials(ref err) => err.description(),
            TerminateJobFlowsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TerminateJobFlowsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon EMR API. Amazon EMR clients implement this trait.
pub trait Emr {
    #[doc="<p>Adds an instance fleet to a running cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x.</p> </note>"]
    fn add_instance_fleet(&self,
                          input: &AddInstanceFleetInput)
                          -> Result<AddInstanceFleetOutput, AddInstanceFleetError>;


    #[doc="<p>Adds one or more instance groups to a running cluster.</p>"]
    fn add_instance_groups(&self,
                           input: &AddInstanceGroupsInput)
                           -> Result<AddInstanceGroupsOutput, AddInstanceGroupsError>;


    #[doc="<p>AddJobFlowSteps adds new steps to a running cluster. A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using SSH to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/ManagementGuide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>A step specifies the location of a JAR file stored either on the master node of the cluster or in Amazon S3. Each step is performed by the main function of the main class of the JAR file. The main class can be specified either in the manifest of the JAR or by using the MainFunction parameter of the step.</p> <p>Amazon EMR executes each step in the order listed. For a step to be considered complete, the main function must exit with a zero exit code and all Hadoop jobs started while the step was running must have completed and run successfully.</p> <p>You can only add steps to a cluster that is in one of the following states: STARTING, BOOTSTRAPPING, RUNNING, or WAITING.</p>"]
    fn add_job_flow_steps(&self,
                          input: &AddJobFlowStepsInput)
                          -> Result<AddJobFlowStepsOutput, AddJobFlowStepsError>;


    #[doc="<p>Adds tags to an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
    fn add_tags(&self, input: &AddTagsInput) -> Result<AddTagsOutput, AddTagsError>;


    #[doc="<p>Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. A maximum of 256 steps are allowed in each CancelSteps request. CancelSteps is idempotent but asynchronous; it does not guarantee a step will be canceled, even if the request is successfully submitted. You can only cancel steps that are in a <code>PENDING</code> state.</p>"]
    fn cancel_steps(&self,
                    input: &CancelStepsInput)
                    -> Result<CancelStepsOutput, CancelStepsError>;


    #[doc="<p>Creates a security configuration, which is stored in the service and can be specified when a cluster is created.</p>"]
    fn create_security_configuration
        (&self,
         input: &CreateSecurityConfigurationInput)
         -> Result<CreateSecurityConfigurationOutput, CreateSecurityConfigurationError>;


    #[doc="<p>Deletes a security configuration.</p>"]
    fn delete_security_configuration
        (&self,
         input: &DeleteSecurityConfigurationInput)
         -> Result<DeleteSecurityConfigurationOutput, DeleteSecurityConfigurationError>;


    #[doc="<p>Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. For information about the cluster steps, see <a>ListSteps</a>.</p>"]
    fn describe_cluster(&self,
                        input: &DescribeClusterInput)
                        -> Result<DescribeClusterOutput, DescribeClusterError>;


    #[doc="<p>This API is deprecated and will eventually be removed. We recommend you use <a>ListClusters</a>, <a>DescribeCluster</a>, <a>ListSteps</a>, <a>ListInstanceGroups</a> and <a>ListBootstrapActions</a> instead.</p> <p>DescribeJobFlows returns a list of job flows that match all of the supplied parameters. The parameters can include a list of job flow IDs, job flow states, and restrictions on job flow creation date and time.</p> <p>Regardless of supplied parameters, only job flows created within the last two months are returned.</p> <p>If no parameters are supplied, then job flows matching either of the following criteria are returned:</p> <ul> <li> <p>Job flows created and completed in the last two weeks</p> </li> <li> <p> Job flows created within the last two months that are in one of the following states: <code>RUNNING</code>, <code>WAITING</code>, <code>SHUTTING_DOWN</code>, <code>STARTING</code> </p> </li> </ul> <p>Amazon EMR can return a maximum of 512 job flow descriptions.</p>"]
    fn describe_job_flows(&self,
                          input: &DescribeJobFlowsInput)
                          -> Result<DescribeJobFlowsOutput, DescribeJobFlowsError>;


    #[doc="<p>Provides the details of a security configuration by returning the configuration JSON.</p>"]
    fn describe_security_configuration
        (&self,
         input: &DescribeSecurityConfigurationInput)
         -> Result<DescribeSecurityConfigurationOutput, DescribeSecurityConfigurationError>;


    #[doc="<p>Provides more detail about the cluster step.</p>"]
    fn describe_step(&self,
                     input: &DescribeStepInput)
                     -> Result<DescribeStepOutput, DescribeStepError>;


    #[doc="<p>Provides information about the bootstrap actions associated with a cluster.</p>"]
    fn list_bootstrap_actions(&self,
                              input: &ListBootstrapActionsInput)
                              -> Result<ListBootstrapActionsOutput, ListBootstrapActionsError>;


    #[doc="<p>Provides the status of all clusters visible to this AWS account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListClusters calls.</p>"]
    fn list_clusters(&self,
                     input: &ListClustersInput)
                     -> Result<ListClustersOutput, ListClustersError>;


    #[doc="<p>Lists all available details about the instance fleets in a cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
    fn list_instance_fleets(&self,
                            input: &ListInstanceFleetsInput)
                            -> Result<ListInstanceFleetsOutput, ListInstanceFleetsError>;


    #[doc="<p>Provides all available details about the instance groups in a cluster.</p>"]
    fn list_instance_groups(&self,
                            input: &ListInstanceGroupsInput)
                            -> Result<ListInstanceGroupsOutput, ListInstanceGroupsError>;


    #[doc="<p>Provides information for all active EC2 instances and EC2 instances terminated in the last 30 days, up to a maximum of 2,000. EC2 instances in any of the following states are considered active: AWAITING_FULFILLMENT, PROVISIONING, BOOTSTRAPPING, RUNNING.</p>"]
    fn list_instances(&self,
                      input: &ListInstancesInput)
                      -> Result<ListInstancesOutput, ListInstancesError>;


    #[doc="<p>Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListSecurityConfigurations calls.</p>"]
    fn list_security_configurations
        (&self,
         input: &ListSecurityConfigurationsInput)
         -> Result<ListSecurityConfigurationsOutput, ListSecurityConfigurationsError>;


    #[doc="<p>Provides a list of steps for the cluster in reverse order unless you specify stepIds with the request.</p>"]
    fn list_steps(&self, input: &ListStepsInput) -> Result<ListStepsOutput, ListStepsError>;


    #[doc="<p>Modifies the target On-Demand and target Spot capacities for the instance fleet with the specified InstanceFleetID within the cluster specified using ClusterID. The call either succeeds or fails atomically.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
    fn modify_instance_fleet(&self,
                             input: &ModifyInstanceFleetInput)
                             -> Result<(), ModifyInstanceFleetError>;


    #[doc="<p>ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. The call will either succeed or fail atomically.</p>"]
    fn modify_instance_groups(&self,
                              input: &ModifyInstanceGroupsInput)
                              -> Result<(), ModifyInstanceGroupsError>;


    #[doc="<p>Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric.</p>"]
    fn put_auto_scaling_policy(&self,
                               input: &PutAutoScalingPolicyInput)
                               -> Result<PutAutoScalingPolicyOutput, PutAutoScalingPolicyError>;


    #[doc="<p>Removes an automatic scaling policy from a specified instance group within an EMR cluster.</p>"]
    fn remove_auto_scaling_policy
        (&self,
         input: &RemoveAutoScalingPolicyInput)
         -> Result<RemoveAutoScalingPolicyOutput, RemoveAutoScalingPolicyError>;


    #[doc="<p>Removes tags from an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p> <p>The following example removes the stack tag with value Prod from a cluster:</p>"]
    fn remove_tags(&self, input: &RemoveTagsInput) -> Result<RemoveTagsOutput, RemoveTagsError>;


    #[doc="<p>RunJobFlow creates and starts running a new cluster (job flow). The cluster runs the steps specified. After the steps complete, the cluster stops and the HDFS partition is lost. To prevent loss of data, configure the last step of the job flow to store results in Amazon S3. If the <a>JobFlowInstancesConfig</a> <code>KeepJobFlowAliveWhenNoSteps</code> parameter is set to <code>TRUE</code>, the cluster transitions to the WAITING state rather than shutting down after the steps have completed. </p> <p>For additional protection, you can set the <a>JobFlowInstancesConfig</a> <code>TerminationProtected</code> parameter to <code>TRUE</code> to lock the cluster and prevent it from being terminated by API call, user intervention, or in the event of a job flow error.</p> <p>A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/Management/Guide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>For long running clusters, we recommend that you periodically store your results.</p> <note> <p>The instance fleets configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions. The RunJobFlow request can contain InstanceFleets parameters or InstanceGroups parameters, but not both.</p> </note>"]
    fn run_job_flow(&self, input: &RunJobFlowInput) -> Result<RunJobFlowOutput, RunJobFlowError>;


    #[doc="<p>SetTerminationProtection locks a cluster (job flow) so the EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. Calling <code>SetTerminationProtection</code> on a cluster is similar to calling the Amazon EC2 <code>DisableAPITermination</code> API on all EC2 instances in a cluster.</p> <p> <code>SetTerminationProtection</code> is used to prevent accidental termination of a cluster and to ensure that in the event of an error, the instances persist so that you can recover any data stored in their ephemeral instance storage.</p> <p> To terminate a cluster that has been locked by setting <code>SetTerminationProtection</code> to <code>true</code>, you must first unlock the job flow by a subsequent call to <code>SetTerminationProtection</code> in which you set the value to <code>false</code>. </p> <p> For more information, see<a href=\"http://docs.aws.amazon.com/emr/latest/ManagementGuide/UsingEMR_TerminationProtection.html\">Managing Cluster Termination</a> in the <i>Amazon EMR Management Guide</i>. </p>"]
    fn set_termination_protection(&self,
                                  input: &SetTerminationProtectionInput)
                                  -> Result<(), SetTerminationProtectionError>;


    #[doc="<p>Sets whether all AWS Identity and Access Management (IAM) users under your account can access the specified clusters (job flows). This action works on running clusters. You can also set the visibility of a cluster when you launch it using the <code>VisibleToAllUsers</code> parameter of <a>RunJobFlow</a>. The SetVisibleToAllUsers action can be called only by an IAM user who created the cluster or the AWS account that owns the cluster.</p>"]
    fn set_visible_to_all_users(&self,
                                input: &SetVisibleToAllUsersInput)
                                -> Result<(), SetVisibleToAllUsersError>;


    #[doc="<p>TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut down, any step not yet completed is canceled and the EC2 instances on which the cluster is running are stopped. Any log files not already saved are uploaded to Amazon S3 if a LogUri was specified when the cluster was created.</p> <p>The maximum number of clusters allowed is 10. The call to <code>TerminateJobFlows</code> is asynchronous. Depending on the configuration of the cluster, it may take up to 1-5 minutes for the cluster to completely terminate and release allocated resources, such as Amazon EC2 instances.</p>"]
    fn terminate_job_flows(&self,
                           input: &TerminateJobFlowsInput)
                           -> Result<(), TerminateJobFlowsError>;
}
/// A client for the Amazon EMR API.
pub struct EmrClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> EmrClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        EmrClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Emr for EmrClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds an instance fleet to a running cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x.</p> </note>"]
    fn add_instance_fleet(&self,
                          input: &AddInstanceFleetInput)
                          -> Result<AddInstanceFleetOutput, AddInstanceFleetError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddInstanceFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddInstanceFleetOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddInstanceFleetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Adds one or more instance groups to a running cluster.</p>"]
    fn add_instance_groups(&self,
                           input: &AddInstanceGroupsInput)
                           -> Result<AddInstanceGroupsOutput, AddInstanceGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddInstanceGroups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddInstanceGroupsOutput>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddInstanceGroupsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>AddJobFlowSteps adds new steps to a running cluster. A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using SSH to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/ManagementGuide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>A step specifies the location of a JAR file stored either on the master node of the cluster or in Amazon S3. Each step is performed by the main function of the main class of the JAR file. The main class can be specified either in the manifest of the JAR or by using the MainFunction parameter of the step.</p> <p>Amazon EMR executes each step in the order listed. For a step to be considered complete, the main function must exit with a zero exit code and all Hadoop jobs started while the step was running must have completed and run successfully.</p> <p>You can only add steps to a cluster that is in one of the following states: STARTING, BOOTSTRAPPING, RUNNING, or WAITING.</p>"]
    fn add_job_flow_steps(&self,
                          input: &AddJobFlowStepsInput)
                          -> Result<AddJobFlowStepsOutput, AddJobFlowStepsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddJobFlowSteps");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddJobFlowStepsOutput>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddJobFlowStepsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Adds tags to an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p>"]
    fn add_tags(&self, input: &AddTagsInput) -> Result<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.AddTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AddTagsOutput>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddTagsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. A maximum of 256 steps are allowed in each CancelSteps request. CancelSteps is idempotent but asynchronous; it does not guarantee a step will be canceled, even if the request is successfully submitted. You can only cancel steps that are in a <code>PENDING</code> state.</p>"]
    fn cancel_steps(&self,
                    input: &CancelStepsInput)
                    -> Result<CancelStepsOutput, CancelStepsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.CancelSteps");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CancelStepsOutput>(String::from_utf8_lossy(&body)
                                                                 .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CancelStepsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a security configuration, which is stored in the service and can be specified when a cluster is created.</p>"]
    fn create_security_configuration
        (&self,
         input: &CreateSecurityConfigurationInput)
         -> Result<CreateSecurityConfigurationOutput, CreateSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "ElasticMapReduce.CreateSecurityConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateSecurityConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateSecurityConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                    .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a security configuration.</p>"]
    fn delete_security_configuration
        (&self,
         input: &DeleteSecurityConfigurationInput)
         -> Result<DeleteSecurityConfigurationOutput, DeleteSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "ElasticMapReduce.DeleteSecurityConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteSecurityConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteSecurityConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                    .as_ref()))
            }
        }
    }


    #[doc="<p>Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. For information about the cluster steps, see <a>ListSteps</a>.</p>"]
    fn describe_cluster(&self,
                        input: &DescribeClusterInput)
                        -> Result<DescribeClusterOutput, DescribeClusterError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.DescribeCluster");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeClusterOutput>(String::from_utf8_lossy(&body)
                                                                     .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeClusterError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>This API is deprecated and will eventually be removed. We recommend you use <a>ListClusters</a>, <a>DescribeCluster</a>, <a>ListSteps</a>, <a>ListInstanceGroups</a> and <a>ListBootstrapActions</a> instead.</p> <p>DescribeJobFlows returns a list of job flows that match all of the supplied parameters. The parameters can include a list of job flow IDs, job flow states, and restrictions on job flow creation date and time.</p> <p>Regardless of supplied parameters, only job flows created within the last two months are returned.</p> <p>If no parameters are supplied, then job flows matching either of the following criteria are returned:</p> <ul> <li> <p>Job flows created and completed in the last two weeks</p> </li> <li> <p> Job flows created within the last two months that are in one of the following states: <code>RUNNING</code>, <code>WAITING</code>, <code>SHUTTING_DOWN</code>, <code>STARTING</code> </p> </li> </ul> <p>Amazon EMR can return a maximum of 512 job flow descriptions.</p>"]
    fn describe_job_flows(&self,
                          input: &DescribeJobFlowsInput)
                          -> Result<DescribeJobFlowsOutput, DescribeJobFlowsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.DescribeJobFlows");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeJobFlowsOutput>(String::from_utf8_lossy(&body)
                                                                      .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeJobFlowsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Provides the details of a security configuration by returning the configuration JSON.</p>"]
    fn describe_security_configuration
        (&self,
         input: &DescribeSecurityConfigurationInput)
         -> Result<DescribeSecurityConfigurationOutput, DescribeSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "ElasticMapReduce.DescribeSecurityConfiguration");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeSecurityConfigurationOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeSecurityConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                      .as_ref()))
            }
        }
    }


    #[doc="<p>Provides more detail about the cluster step.</p>"]
    fn describe_step(&self,
                     input: &DescribeStepInput)
                     -> Result<DescribeStepOutput, DescribeStepError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.DescribeStep");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeStepOutput>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStepError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Provides information about the bootstrap actions associated with a cluster.</p>"]
    fn list_bootstrap_actions(&self,
                              input: &ListBootstrapActionsInput)
                              -> Result<ListBootstrapActionsOutput, ListBootstrapActionsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListBootstrapActions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListBootstrapActionsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBootstrapActionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Provides the status of all clusters visible to this AWS account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListClusters calls.</p>"]
    fn list_clusters(&self,
                     input: &ListClustersInput)
                     -> Result<ListClustersOutput, ListClustersError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListClusters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListClustersOutput>(String::from_utf8_lossy(&body)
                                                                  .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListClustersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all available details about the instance fleets in a cluster.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
    fn list_instance_fleets(&self,
                            input: &ListInstanceFleetsInput)
                            -> Result<ListInstanceFleetsOutput, ListInstanceFleetsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListInstanceFleets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListInstanceFleetsOutput>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListInstanceFleetsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Provides all available details about the instance groups in a cluster.</p>"]
    fn list_instance_groups(&self,
                            input: &ListInstanceGroupsInput)
                            -> Result<ListInstanceGroupsOutput, ListInstanceGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListInstanceGroups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListInstanceGroupsOutput>(String::from_utf8_lossy(&body)
                                                                        .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListInstanceGroupsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Provides information for all active EC2 instances and EC2 instances terminated in the last 30 days, up to a maximum of 2,000. EC2 instances in any of the following states are considered active: AWAITING_FULFILLMENT, PROVISIONING, BOOTSTRAPPING, RUNNING.</p>"]
    fn list_instances(&self,
                      input: &ListInstancesInput)
                      -> Result<ListInstancesOutput, ListInstancesError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListInstancesOutput>(String::from_utf8_lossy(&body)
                                                                   .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListInstancesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple ListSecurityConfigurations calls.</p>"]
    fn list_security_configurations
        (&self,
         input: &ListSecurityConfigurationsInput)
         -> Result<ListSecurityConfigurationsOutput, ListSecurityConfigurationsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "ElasticMapReduce.ListSecurityConfigurations");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListSecurityConfigurationsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListSecurityConfigurationsError::from_body(String::from_utf8_lossy(&body)
                                                                   .as_ref()))
            }
        }
    }


    #[doc="<p>Provides a list of steps for the cluster in reverse order unless you specify stepIds with the request.</p>"]
    fn list_steps(&self, input: &ListStepsInput) -> Result<ListStepsOutput, ListStepsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ListSteps");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListStepsOutput>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListStepsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the target On-Demand and target Spot capacities for the instance fleet with the specified InstanceFleetID within the cluster specified using ClusterID. The call either succeeds or fails atomically.</p> <note> <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions.</p> </note>"]
    fn modify_instance_fleet(&self,
                             input: &ModifyInstanceFleetInput)
                             -> Result<(), ModifyInstanceFleetError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ModifyInstanceFleet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ModifyInstanceFleetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. The call will either succeed or fail atomically.</p>"]
    fn modify_instance_groups(&self,
                              input: &ModifyInstanceGroupsInput)
                              -> Result<(), ModifyInstanceGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.ModifyInstanceGroups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ModifyInstanceGroupsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric.</p>"]
    fn put_auto_scaling_policy(&self,
                               input: &PutAutoScalingPolicyInput)
                               -> Result<PutAutoScalingPolicyOutput, PutAutoScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.PutAutoScalingPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutAutoScalingPolicyOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutAutoScalingPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes an automatic scaling policy from a specified instance group within an EMR cluster.</p>"]
    fn remove_auto_scaling_policy
        (&self,
         input: &RemoveAutoScalingPolicyInput)
         -> Result<RemoveAutoScalingPolicyOutput, RemoveAutoScalingPolicyError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.RemoveAutoScalingPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RemoveAutoScalingPolicyOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RemoveAutoScalingPolicyError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }


    #[doc="<p>Removes tags from an Amazon EMR resource. Tags make it easier to associate clusters in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. For more information, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/DeveloperGuide/emr-plan-tags.html\">Tagging Amazon EMR Resources</a>. </p> <p>The following example removes the stack tag with value Prod from a cluster:</p>"]
    fn remove_tags(&self, input: &RemoveTagsInput) -> Result<RemoveTagsOutput, RemoveTagsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.RemoveTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RemoveTagsOutput>(String::from_utf8_lossy(&body)
                                                                .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RemoveTagsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>RunJobFlow creates and starts running a new cluster (job flow). The cluster runs the steps specified. After the steps complete, the cluster stops and the HDFS partition is lost. To prevent loss of data, configure the last step of the job flow to store results in Amazon S3. If the <a>JobFlowInstancesConfig</a> <code>KeepJobFlowAliveWhenNoSteps</code> parameter is set to <code>TRUE</code>, the cluster transitions to the WAITING state rather than shutting down after the steps have completed. </p> <p>For additional protection, you can set the <a>JobFlowInstancesConfig</a> <code>TerminationProtected</code> parameter to <code>TRUE</code> to lock the cluster and prevent it from being terminated by API call, user intervention, or in the event of a job flow error.</p> <p>A maximum of 256 steps are allowed in each job flow.</p> <p>If your cluster is long-running (such as a Hive data warehouse) or complex, you may require more than 256 steps to process your data. You can bypass the 256-step limitation in various ways, including using the SSH shell to connect to the master node and submitting queries directly to the software running on the master node, such as Hive and Hadoop. For more information on how to do this, see <a href=\"http://docs.aws.amazon.com/ElasticMapReduce/latest/Management/Guide/AddMoreThan256Steps.html\">Add More than 256 Steps to a Cluster</a> in the <i>Amazon EMR Management Guide</i>.</p> <p>For long running clusters, we recommend that you periodically store your results.</p> <note> <p>The instance fleets configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x versions. The RunJobFlow request can contain InstanceFleets parameters or InstanceGroups parameters, but not both.</p> </note>"]
    fn run_job_flow(&self, input: &RunJobFlowInput) -> Result<RunJobFlowOutput, RunJobFlowError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.RunJobFlow");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RunJobFlowOutput>(String::from_utf8_lossy(&body)
                                                                .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RunJobFlowError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>SetTerminationProtection locks a cluster (job flow) so the EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. Calling <code>SetTerminationProtection</code> on a cluster is similar to calling the Amazon EC2 <code>DisableAPITermination</code> API on all EC2 instances in a cluster.</p> <p> <code>SetTerminationProtection</code> is used to prevent accidental termination of a cluster and to ensure that in the event of an error, the instances persist so that you can recover any data stored in their ephemeral instance storage.</p> <p> To terminate a cluster that has been locked by setting <code>SetTerminationProtection</code> to <code>true</code>, you must first unlock the job flow by a subsequent call to <code>SetTerminationProtection</code> in which you set the value to <code>false</code>. </p> <p> For more information, see<a href=\"http://docs.aws.amazon.com/emr/latest/ManagementGuide/UsingEMR_TerminationProtection.html\">Managing Cluster Termination</a> in the <i>Amazon EMR Management Guide</i>. </p>"]
    fn set_termination_protection(&self,
                                  input: &SetTerminationProtectionInput)
                                  -> Result<(), SetTerminationProtectionError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.SetTerminationProtection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetTerminationProtectionError::from_body(String::from_utf8_lossy(&body)
                                                                 .as_ref()))
            }
        }
    }


    #[doc="<p>Sets whether all AWS Identity and Access Management (IAM) users under your account can access the specified clusters (job flows). This action works on running clusters. You can also set the visibility of a cluster when you launch it using the <code>VisibleToAllUsers</code> parameter of <a>RunJobFlow</a>. The SetVisibleToAllUsers action can be called only by an IAM user who created the cluster or the AWS account that owns the cluster.</p>"]
    fn set_visible_to_all_users(&self,
                                input: &SetVisibleToAllUsersInput)
                                -> Result<(), SetVisibleToAllUsersError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.SetVisibleToAllUsers");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetVisibleToAllUsersError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut down, any step not yet completed is canceled and the EC2 instances on which the cluster is running are stopped. Any log files not already saved are uploaded to Amazon S3 if a LogUri was specified when the cluster was created.</p> <p>The maximum number of clusters allowed is 10. The call to <code>TerminateJobFlows</code> is asynchronous. Depending on the configuration of the cluster, it may take up to 1-5 minutes for the cluster to completely terminate and release allocated resources, such as Amazon EC2 instances.</p>"]
    fn terminate_job_flows(&self,
                           input: &TerminateJobFlowsInput)
                           -> Result<(), TerminateJobFlowsError> {
        let mut request = SignedRequest::new("POST", "elasticmapreduce", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ElasticMapReduce.TerminateJobFlows");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TerminateJobFlowsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
