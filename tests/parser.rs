extern crate protoparse;

use std::path::Path;

use protoparse::ast::*;
use protoparse::parser::{parse, parse_from_file};

#[test]
fn should_parse_syntax() {
    let input = "syntax = \"proto3\";".to_string();
    let result = parse(&input).unwrap();

    assert_eq!(result.syntax, Syntax::V3);
}

#[test]
fn should_parse_unmodified_import() {
    let input = min_file() + "import \"foobar.proto\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Default);
    assert_eq!(result.imports[0].name, "foobar.proto".to_string());
}

#[test]
fn should_parse_weak_import() {
    let input = min_file() + "import weak \"imp.proto\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Weak);
    assert_eq!(result.imports[0].name, "imp.proto".to_string());
}

#[test]
fn should_parse_public_import() {
    let input = min_file() + "import public \"header.proto\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Public);
    assert_eq!(result.imports[0].name, "header.proto".to_string());
}

#[test]
fn should_parse_empty_stmts() {
    let input = min_file() + ";;;;";
    parse(&input).unwrap();
}

#[test]
fn parse_package_with_simple_name() {
    let input = min_file() + "package foo;";
    let result = parse(&input).unwrap();

    assert_eq!(result.packages.len(), 1);
    assert_eq!(result.packages[0].full_ident.idents.len(), 1);
    assert_eq!(result.packages[0].full_ident.idents[0], "foo".to_string());
}

#[test]
fn parse_package_with_full_name() {
    let input = min_file() + "package foo.bar.baz;";
    let result = parse(&input).unwrap();

    assert_eq!(result.packages.len(), 1);
    assert_eq!(result.packages[0].full_ident.idents.len(), 3);
    assert_eq!(result.packages[0].full_ident.idents[0], "foo".to_string());
    assert_eq!(result.packages[0].full_ident.idents[1], "bar".to_string());
    assert_eq!(result.packages[0].full_ident.idents[2], "baz".to_string());
}

#[test]
fn parse_not_package_without_name() {
    let input = min_file() + "package;";
    assert!(parse(&input).is_err());
}

#[test]
fn parse_option_with_simple_ident_and_number_value() {
    let input = min_file() + "option my_option = 42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].full_ident.idents.len(), 1);
    assert_eq!(
        result.options[0].full_ident.idents[0],
        "my_option".to_string()
    );
    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_string_value() {
    let input = min_file() + "option my_option = \"strOpt\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(
        result.options[0].constant,
        ConstantValue::StringValue("strOpt".to_string())
    );
}

#[test]
fn parse_option_bool_value() {
    let input = min_file() + "option my_option = true;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::BoolValue(true));
}

#[test]
fn parse_option_full_ident_value() {
    let input = min_file() + "option my_option = my.ident.value;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(
        result.options[0].constant,
        ConstantValue::IdentValue(FullIdent::new(vec![
            "my".to_string(),
            "ident".to_string(),
            "value".to_string()
        ]))
    );
}

#[test]
fn parse_option_with_full_ident_name() {
    //not really allowed in the grammar, () are missing
    let input = min_file() + "option my_option.full.ident.name = 42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].full_ident.idents.len(), 4);
    assert_eq!(
        result.options[0].full_ident.idents[0],
        "my_option".to_string()
    );
    assert_eq!(result.options[0].full_ident.idents[1], "full".to_string());
    assert_eq!(result.options[0].full_ident.idents[2], "ident".to_string());
    assert_eq!(result.options[0].full_ident.idents[3], "name".to_string());

    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_with_full_ident_name_in_paren() {
    //not really allowed in the grammar, () are missing
    let input = min_file() + "option (my_option.full.ident.name ) = 42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].full_ident.idents.len(), 4);
    assert_eq!(
        result.options[0].full_ident.idents[0],
        "my_option".to_string()
    );
    assert_eq!(result.options[0].full_ident.idents[1], "full".to_string());
    assert_eq!(result.options[0].full_ident.idents[2], "ident".to_string());
    assert_eq!(result.options[0].full_ident.idents[3], "name".to_string());

    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_with_plus_prefixed_number() {
    let input = min_file() + "option foo = +42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_with_minus_prefixed_number() {
    let input = min_file() + "option foo = -42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(
        result.options[0].constant,
        ConstantValue::NumberValue(-42.0)
    );
}

#[test]
fn parse_empty_service() {
    let input = min_file()
        + "service EmptyService {
    }
    ";
    let result = parse(&input).unwrap();

    assert_eq!(result.services.len(), 1);
    assert_eq!(result.services[0].name, "EmptyService");
    assert_eq!(result.services[0].rpcs.len(), 0);
}

#[test]
fn parse_service() {
    let input = min_file()
        + "service GatewayService {
      rpc GetGreeting(GetGreetingReq) returns (GetGreetingRsp);
    }
    ";
    let result = parse(&input).unwrap();

    assert_eq!(result.services.len(), 1);
    assert_eq!(result.services[0].name, "GatewayService");
    assert_eq!(result.services[0].rpcs.len(), 1);
    assert_eq!(result.services[0].rpcs[0].name, "GetGreeting");
    assert_eq!(result.services[0].rpcs[0].request_type.idents.len(), 1);
    assert_eq!(
        result.services[0].rpcs[0].request_type.idents[0],
        "GetGreetingReq"
    );

    assert_eq!(result.services[0].rpcs[0].response_type.idents.len(), 1);
    assert_eq!(
        result.services[0].rpcs[0].response_type.idents[0],
        "GetGreetingRsp"
    );
}

#[test]
fn parse_service_with_curly_brace_termination() {
    let input = min_file()
        + "service GatewayService {
      rpc GetGreeting(GetGreetingReq) returns (GetGreetingRsp) {}
    }
    ";
    let result = parse(&input).unwrap();

    assert_eq!(result.services.len(), 1);
    assert_eq!(result.services[0].name, "GatewayService");
    assert_eq!(result.services[0].rpcs.len(), 1);
    assert_eq!(result.services[0].rpcs[0].name, "GetGreeting");
    assert_eq!(result.services[0].rpcs[0].request_type.idents.len(), 1);
    assert_eq!(
        result.services[0].rpcs[0].request_type.idents[0],
        "GetGreetingReq"
    );

    assert_eq!(result.services[0].rpcs[0].response_type.idents.len(), 1);
    assert_eq!(
        result.services[0].rpcs[0].response_type.idents[0],
        "GetGreetingRsp"
    );
}

#[test]
fn parse_service_with_3_rpcs() {
    let input = min_file()
        + "service ServiceWith3 {
      rpc Rpc1(Req1) returns (Rsp1);
      rpc Rpc2(Req2) returns (Rsp2);
      rpc Rpc3(Req3) returns (Rsp3);
    }
    ";
    let result = parse(&input).unwrap();

    assert_eq!(result.services.len(), 1);
    assert_eq!(result.services[0].name, "ServiceWith3");

    assert_eq!(result.services[0].rpcs.len(), 3);
    assert_eq!(result.services[0].rpcs[0].name, "Rpc1");
    assert_eq!(result.services[0].rpcs[0].request_type.idents.len(), 1);
    assert_eq!(result.services[0].rpcs[0].request_type.idents[0], "Req1");
    assert_eq!(result.services[0].rpcs[0].response_type.idents.len(), 1);
    assert_eq!(result.services[0].rpcs[0].response_type.idents[0], "Rsp1");

    assert_eq!(result.services[0].rpcs[1].name, "Rpc2");
    assert_eq!(result.services[0].rpcs[1].request_type.idents.len(), 1);
    assert_eq!(result.services[0].rpcs[1].request_type.idents[0], "Req2");
    assert_eq!(result.services[0].rpcs[1].response_type.idents.len(), 1);
    assert_eq!(result.services[0].rpcs[1].response_type.idents[0], "Rsp2");

    assert_eq!(result.services[0].rpcs[2].name, "Rpc3");
    assert_eq!(result.services[0].rpcs[2].request_type.idents.len(), 1);
    assert_eq!(result.services[0].rpcs[2].request_type.idents[0], "Req3");
    assert_eq!(result.services[0].rpcs[2].response_type.idents.len(), 1);
    assert_eq!(result.services[0].rpcs[2].response_type.idents[0], "Rsp3");
}

#[test]
fn parse_empty_message() {
    let input = min_file() + "message EmptyMessage {}";

    let result = parse(&input).unwrap();
    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0].name, "EmptyMessage");
}

#[test]
fn parse_message_with_fields() {
    let input = min_file()
        + "message MessageWithField {
        string foo = 1;
        repeated string bar = 2;
        double baz = 3;
        bool boo = 4;
    }";

    let result = parse(&input).unwrap();
    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0].name, "MessageWithField");

    assert_eq!(result.messages[0].fields.len(), 4);
    assert_eq!(result.messages[0].fields[0].field_type, Type::String);
    assert_eq!(result.messages[0].fields[0].name, "foo");
    assert_eq!(result.messages[0].fields[0].repeated, false);
    assert_eq!(result.messages[0].fields[0].field_number, 1);

    assert_eq!(result.messages[0].fields[1].field_type, Type::String);
    assert_eq!(result.messages[0].fields[1].name, "bar");
    assert_eq!(result.messages[0].fields[1].repeated, true);
    assert_eq!(result.messages[0].fields[1].field_number, 2);

    assert_eq!(result.messages[0].fields[2].field_type, Type::Double);
    assert_eq!(result.messages[0].fields[2].name, "baz");
    assert_eq!(result.messages[0].fields[2].repeated, false);
    assert_eq!(result.messages[0].fields[2].field_number, 3);

    assert_eq!(result.messages[0].fields[3].field_type, Type::Bool);
    assert_eq!(result.messages[0].fields[3].name, "boo");
    assert_eq!(result.messages[0].fields[3].repeated, false);
    assert_eq!(result.messages[0].fields[3].field_number, 4);
}

#[test]
fn parse_message_with_keyword_identifier() {
    let input = min_file()
        + "message HelloReply {
      string message = 1;
    }";

    let result = parse(&input).unwrap();

    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0].name, "HelloReply");

    assert_eq!(result.messages[0].fields.len(), 1);
    assert_eq!(result.messages[0].fields[0].field_type, Type::String);
    assert_eq!(result.messages[0].fields[0].name, "message");
    assert_eq!(result.messages[0].fields[0].repeated, false);
    assert_eq!(result.messages[0].fields[0].field_number, 1);
}

#[test]
fn parse_simple_proto_file() {
    let input = r#"syntax = "proto3";

    option go_package = "github.com/some/protoForGo";

    package gateway;

    service GatewayService {
      rpc GetGreeting(GetGreetingReq) returns (GetGreetingRsp) {}
    }

    message GetGreetingReq {
      string YourName = 1;
    }

    message GetGreetingRsp {
      string Greeting = 1;
  }"#
    .to_string();

    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.packages.len(), 1);
    assert_eq!(result.services.len(), 1);
    assert_eq!(result.messages.len(), 2);
}

#[test]
fn should_parse_from_file_simple_example() {
    parse_from_file(Path::new("tests/testdata/simple.proto")).unwrap();
}

#[test]
#[should_panic]
fn should_error_if_file_does_not_exist() {
    parse_from_file(Path::new("does/not/exist.proto")).unwrap();
}

//taken from: https://github.com/grpc/grpc-go/tree/master/examples/helloworld
#[test]
fn should_parse_helloworld_example() {
    parse_from_file(Path::new("tests/testdata/helloworld.proto")).unwrap();
}

// helper methods

fn min_file() -> String {
    return "syntax = \"proto3\";".to_string();
}
