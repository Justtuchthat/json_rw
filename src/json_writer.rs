use crate::json_type::JSONtype;
use string_builder::Builder;

pub fn write_json(json: JSONtype) -> String {
    /* JSON = Value
    Value = String
    Value = Int
    Value = Float
    Value = Bool
    Value = Null
    Value = List
    Value = Object
    List = '[' + ListItems + ']'
    ListItems = Value + ',' + ListItems
    ListItems = Value
    Object = '{' + ObjectItems + '}'
    ObjectItems = ObjectItem + ',' + ObjectItems
    ObjectItems = ObjectItem
    ObjectItem = String + ':' + Value */

    write_value(json, 0).string().unwrap()
}

fn write_value(json: JSONtype, indentation: usize) -> Builder {
    let mut s_builder = Builder::default();
    s_builder.append(std::iter::repeat('\t').take(indentation).collect::<String>());
    match json {
        JSONtype::Bool(true) => {s_builder.append("true");},
        JSONtype::Bool(false) => {s_builder.append("false");},
        JSONtype::Null => {s_builder.append("null");},
        JSONtype::String(s) => {
            s_builder.append('"');
            s_builder.append(s);
            s_builder.append('"');
        },
        JSONtype::Int(i) => {s_builder.append(i.to_string());},
        JSONtype::Float(f) => {s_builder.append(f.to_string());},
        JSONtype::List(jsonvec) => {
            s_builder.append("[\n");
            for list_item in jsonvec {
                s_builder.append(write_value(list_item, indentation+1).string().unwrap());
                s_builder.append(",\n");
            };
            let mut s = s_builder.string().unwrap();
            let _ = s.pop();
            let _ = s.pop();
            s_builder = Builder::default();
            s_builder.append(s);
            s_builder.append('\n');
            s_builder.append(std::iter::repeat('\t').take(indentation).collect::<String>());
            s_builder.append(']');
        }
        JSONtype::Object(jsonmap) => {
            s_builder.append("{\n");
            for (key, jsonvalue) in jsonmap {
                s_builder.append(std::iter::repeat('\t').take(indentation+1).collect::<String>());
                s_builder.append('"');
                s_builder.append(key);
                s_builder.append("\":\n");
                s_builder.append(write_value(jsonvalue, indentation+1).string().unwrap());
                s_builder.append(",\n");
            }
            let mut s = s_builder.string().unwrap();
            let _ = s.pop();
            let _ = s.pop();
            s_builder = Builder::default();
            s_builder.append(s);
            s_builder.append('\n');
            s_builder.append(std::iter::repeat('\t').take(indentation).collect::<String>());
            s_builder.append('}');
        },
    };
    s_builder
}