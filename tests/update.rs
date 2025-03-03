mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/update.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_update_step1() {
    let data = r#"
        {
            "messages":[
                {
                    "content":{"text":"1"},"content_type":"text"
                },
                {
                    "content":{"text":"4"},"content_type":"text"
                } 
            ],
            "next_flow":null,
            "memories":[],
            "next_step":"end"
        }"#;
    let msg = format_message(gen_event(""), "step1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step2() {
    let data = r#"{"messages":[ {"content": [{"test": 1}, 2, 3, 4, 5], "content_type":"array"}, {"content": [1, 2, 3, 4, 5], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "step2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step3() {
    let data = r#"{"messages":[ {"content": [1], "content_type":"array"}, {"content": [2], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "step3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step4() {
    let data = r#"{"messages":[ {"content": [1], "content_type":"array"}, {"content": [1, 2], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "step4");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_update_step5() {
    let data = r#"{"messages":[ {"content": [1, 2], "content_type":"array"}, {"content": [1], "content_type":"array"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "step5");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
