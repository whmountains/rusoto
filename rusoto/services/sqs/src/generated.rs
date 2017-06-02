#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto_core::request::DispatchSignedRequest;
        use rusoto_core::region;

        use std::fmt;
        use std::error::Error;
        use rusoto_core::request::HttpDispatchError;
        use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
    
use std::str::FromStr;
            use xml::EventReader;
            use xml::reader::ParserConfig;
            use rusoto_core::param::{Params, ServiceParams};
            use rusoto_core::signature::SignedRequest;
            use xml::reader::XmlEvent;
            use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
            use rusoto_core::xmlutil::{characters, end_element, start_element, skip_tree, peek_at_name};
            use rusoto_core::xmlerror::*;

            enum DeserializerNext {
                Close,
                Skip,
                Element(String),
        }
pub type AWSAccountIdList = Vec<String>;

            /// Serialize `AWSAccountIdList` contents to a `SignedRequest`.
            struct AWSAccountIdListSerializer;
            impl AWSAccountIdListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &AWSAccountIdList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.{}", name, index+1);
params.put(&key, &obj);
}
                }
            }
            
pub type ActionNameList = Vec<String>;

            /// Serialize `ActionNameList` contents to a `SignedRequest`.
            struct ActionNameListSerializer;
            impl ActionNameListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &ActionNameList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.{}", name, index+1);
params.put(&key, &obj);
}
                }
            }
            
#[doc="<p/>"]
#[derive(Default,Debug,Clone)]
            pub struct AddPermissionRequest {
                #[doc="<p>The AWS account number of the <a href=\"http://docs.aws.amazon.com/general/latest/gr/glos-chap.html#P\">principal</a> who is given permission. The principal must have an AWS account, but does not need to be signed up for Amazon SQS. For information about locating the AWS account identification, see <a href=\"http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/AWSCredentials.html\">Your AWS Identifiers</a> in the <i>Amazon SQS Developer Guide</i>.</p>"]
pub aws_account_ids: AWSAccountIdList,
#[doc="<p>The action the client wants to allow for the specified principal. The following values are valid:</p> <ul> <li> <p> <code>*</code> </p> </li> <li> <p> <code>ChangeMessageVisibility</code> </p> </li> <li> <p> <code>DeleteMessage</code> </p> </li> <li> <p> <code>GetQueueAttributes</code> </p> </li> <li> <p> <code>GetQueueUrl</code> </p> </li> <li> <p> <code>ReceiveMessage</code> </p> </li> <li> <p> <code>SendMessage</code> </p> </li> </ul> <p>For more information about these actions, see <a href=\"http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html#PermissionTypes\">Understanding Permissions</a> in the <i>Amazon SQS Developer Guide</i>.</p> <p>Specifying <code>SendMessage</code>, <code>DeleteMessage</code>, or <code>ChangeMessageVisibility</code> for <code>ActionName.n</code> also grants permissions for the corresponding batch versions of those actions: <code>SendMessageBatch</code>, <code>DeleteMessageBatch</code>, and <code>ChangeMessageVisibilityBatch</code>.</p>"]
pub actions: ActionNameList,
#[doc="<p>The unique identification of the permission you're setting (for example, <code>AliceSendMessage</code>). Maximum 80 characters. Allowed characters include alphanumeric characters, hyphens (<code>-</code>), and underscores (<code>_</code>).</p>"]
pub label: String,
#[doc="<p>The URL of the Amazon SQS queue to which permissions are added.</p> <p>Queue URLs are case-sensitive.</p>"]
pub queue_url: String,
            }
            

            /// Serialize `AddPermissionRequest` contents to a `SignedRequest`.
            struct AddPermissionRequestSerializer;
            impl AddPermissionRequestSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &AddPermissionRequest) {
                    let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AWSAccountIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AWSAccountIds"),
                &obj.aws_account_ids,
            );
ActionNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Actions"),
                &obj.actions,
            );
params.put(&format!("{}{}", prefix, "Label"), &obj.label);
params.put(&format!("{}{}", prefix, "QueueUrl"), &obj.queue_url);
        
                }
            }
            
pub type AttributeNameList = Vec<QueueAttributeName>;

            /// Serialize `AttributeNameList` contents to a `SignedRequest`.
            struct AttributeNameListSerializer;
            impl AttributeNameListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &AttributeNameList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.{}", name, index+1);
params.put(&key, &obj);
}
                }
            }
            
#[doc="<p>This is used in the responses of batch API to give a detailed description of the result of an action on each entry in the request.</p>"]
#[derive(Default,Debug,Clone)]
            pub struct BatchResultErrorEntry {
                #[doc="<p>An error code representing why the action failed on this entry.</p>"]
pub code: String,
#[doc="<p>The <code>Id</code> of an entry in a batch request.</p>"]
pub id: String,
#[doc="<p>A message explaining why the action failed on this entry.</p>"]
pub message: Option<String>,
#[doc="<p>Specifies whether the error happened due to the sender's fault.</p>"]
pub sender_fault: Boolean,
            }
            
struct BatchResultErrorEntryDeserializer;
            impl BatchResultErrorEntryDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<BatchResultErrorEntry, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let code: String;
let id: String;
let message: Option<String>;
let sender_fault: bool;
        
        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Code" => {
                code = try!(StringDeserializer::deserialize("Code", stack));
            }
"Id" => {
                id = try!(StringDeserializer::deserialize("Id", stack));
            }
"Message" => {
                message = Some(try!(StringDeserializer::deserialize("Message", stack)));
            }
"SenderFault" => {
                sender_fault = try!(BooleanDeserializer::deserialize("SenderFault", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));
        let mut obj = BatchResultErrorEntry{
            code: code,
id: id,
message: message,
sender_fault: sender_fault,
        }; // needs deserializer: xmlpayloadparser                
        Ok(obj)
        
                }
            }
pub type BatchResultErrorEntryList = Vec<BatchResultErrorEntry>;
struct BatchResultErrorEntryListDeserializer;
            impl BatchResultErrorEntryListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<BatchResultErrorEntryList, XmlParseError> {
                    
        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false
            };

            if consume_next_tag {
                obj.push(try!(BatchResultErrorEntryDeserializer::deserialize(tag_name, stack)));
            } else {
                break
            }

        }

        Ok(obj)
        
                }
            }
pub type Binary = Vec<u8>;
struct BinaryDeserializer;
            impl BinaryDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Binary, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack)).into_bytes();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type BinaryList = Vec<Binary>;
struct BinaryListDeserializer;
            impl BinaryListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<BinaryList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "BinaryListValue" {
                        obj.push(try!(BinaryDeserializer::deserialize("BinaryListValue", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }

            /// Serialize `BinaryList` contents to a `SignedRequest`.
            struct BinaryListSerializer;
            impl BinaryListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &BinaryList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.member.{}", name, index+1);
params.put(&key, ::std::str::from_utf8(&obj).unwrap());
}
                }
            }
            
pub type Boolean = bool;
struct BooleanDeserializer;
            impl BooleanDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Boolean, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p/>"]
#[derive(Default,Debug,Clone)]
            pub struct ChangeMessageVisibilityBatchRequest {
                #[doc="<p>A list of receipt handles of the messages for which the visibility timeout must be changed.</p>"]
pub entries: ChangeMessageVisibilityBatchRequestEntryList,
#[doc="<p>The URL of the Amazon SQS queue whose messages' visibility is changed.</p> <p>Queue URLs are case-sensitive.</p>"]
pub queue_url: String,
            }
            

            /// Serialize `ChangeMessageVisibilityBatchRequest` contents to a `SignedRequest`.
            struct ChangeMessageVisibilityBatchRequestSerializer;
            impl ChangeMessageVisibilityBatchRequestSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequest) {
                    let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        ChangeMessageVisibilityBatchRequestEntryListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Entries"),
                &obj.entries,
            );
params.put(&format!("{}{}", prefix, "QueueUrl"), &obj.queue_url);
        
                }
            }
            
#[doc="<p>Encloses a receipt handle and an entry id for each message in <code> <a>ChangeMessageVisibilityBatch</a> </code>.</p> <important> <p>All of the following list parameters must be prefixed with <code>ChangeMessageVisibilityBatchRequestEntry.n</code>, where <code>n</code> is an integer value starting with <code>1</code>. For example, a parameter list for this action might look like this:</p> </important> <p> <code>&amp;amp;ChangeMessageVisibilityBatchRequestEntry.1.Id=change_visibility_msg_2</code> </p> <p> <code>&amp;amp;ChangeMessageVisibilityBatchRequestEntry.1.ReceiptHandle=&lt;replaceable&gt;Your_Receipt_Handle&lt;/replaceable&gt;</code> </p> <p> <code>&amp;amp;ChangeMessageVisibilityBatchRequestEntry.1.VisibilityTimeout=45</code> </p>"]
#[derive(Default,Debug,Clone)]
            pub struct ChangeMessageVisibilityBatchRequestEntry {
                #[doc="<p>An identifier for this particular receipt handle used to communicate the result.</p> <note> <p>The <code>Id</code>s of a batch request need to be unique within a request</p> </note>"]
pub id: String,
#[doc="<p>A receipt handle.</p>"]
pub receipt_handle: String,
#[doc="<p>The new value (in seconds) for the message's visibility timeout.</p>"]
pub visibility_timeout: Option<Integer>,
            }
            

            /// Serialize `ChangeMessageVisibilityBatchRequestEntry` contents to a `SignedRequest`.
            struct ChangeMessageVisibilityBatchRequestEntrySerializer;
            impl ChangeMessageVisibilityBatchRequestEntrySerializer {
                fn serialize(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntry) {
                    let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Id"), &obj.id);
params.put(&format!("{}{}", prefix, "ReceiptHandle"), &obj.receipt_handle);
if let Some(ref field_value) = obj.visibility_timeout {
                params.put(&format!("{}{}", prefix, "VisibilityTimeout"), &field_value.to_string());
            }
        
                }
            }
            
pub type ChangeMessageVisibilityBatchRequestEntryList = Vec<ChangeMessageVisibilityBatchRequestEntry>;

            /// Serialize `ChangeMessageVisibilityBatchRequestEntryList` contents to a `SignedRequest`.
            struct ChangeMessageVisibilityBatchRequestEntryListSerializer;
            impl ChangeMessageVisibilityBatchRequestEntryListSerializer {
                fn serialize(params: &mut Params, name: &str, obj: &ChangeMessageVisibilityBatchRequestEntryList) {
                    for (index, obj) in obj.iter().enumerate() {
                    let key = format!("{}.{}", name, index+1);
ChangeMessageVisibilityBatchRequestEntrySerializer::serialize(params, &key, obj);
}
                }
            }
            
#[doc="<p>For each message in the batch, the response contains a <code> <a>ChangeMessageVisibilityBatchResultEntry</a> </code> tag if the message succeeds or a <code> <a>BatchResultErrorEntry</a> </code> tag if the message fails.</p>"]
#[derive(Default,Debug,Clone)]
            pub struct ChangeMessageVisibilityBatchResult {
                #[doc="<p>A list of <code> <a>BatchResultErrorEntry</a> </code> items.</p>"]
pub failed: BatchResultErrorEntryList,
#[doc="<p>A list of <code> <a>ChangeMessageVisibilityBatchResultEntry</a> </code> items.</p>"]
pub successful: ChangeMessageVisibilityBatchResultEntryList,
            }
            
