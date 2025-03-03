mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;

use support::tools::{gen_context, gen_event, message_to_jsonvalue, read_file};

fn format_message(event: Event, step: &str) -> MessageData {
    let text = read_file("CSML/basic_test/break.csml".to_owned()).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn break_test_0() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "start");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_1() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "break_test_0");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_2() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "break_test_1");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_3() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type":"text"}, {"content":{"text":"Hello"}, "content_type":"text"}, {"content":{"text":"World"}, "content_type": "text"}], "next_flow":null, "next_step":null}"#;
    let msg = format_message(gen_event(""), "break_test_2");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn break_test_4() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"Hello"}, "content_type":"text"}], "next_flow":null, "next_step":"foo"}"#;
    let msg = format_message(gen_event(""), "break_test_3");

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
