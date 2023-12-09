use super::to_ts_type;
use type_reflect_core::{Inflectable, Inflection, NamedField};

pub fn struct_member(member: &NamedField, inflection: Inflection) -> String {
    let name = &member.name.inflect(inflection);
    match &member.type_ {
        type_reflect_core::Type::Option(t) => {
            let value = to_ts_type(&t);
            format!("{name}?: {value};", name = name, value = value)
        }
        t => {
            let value = to_ts_type(&t);
            format!("{name}: {value};", name = name, value = value)
        }
    }
}

pub fn struct_members(members: &Vec<NamedField>, inflection: Inflection) -> String {
    let members: Vec<String> = members
        .into_iter()
        .map(|member| struct_member(member, inflection))
        .collect();
    members.join("\n  ")
}

pub fn struct_impl(name: &str, members: &Vec<NamedField>, inflection: Inflection) -> String {
    let members = struct_members(members, inflection);
    return format!(
        r#"

export type {name} = {{
    {members}
}};
        "#,
        name = name,
        members = members
    );
}
