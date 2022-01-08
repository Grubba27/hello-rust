/// here where you create the base of your object
#[derive(Clone)]
struct ComplexObject {
    parts: Vec<String>,
}

impl ComplexObject {
    fn create() -> ComplexObject {
        ComplexObject { parts: Vec::new() }
    }
}

trait Builder {
    fn push_part(&mut self, value: &str);
    fn get_build(&mut self) -> ComplexObject;
}

/// this is what you use

struct UsableBuilder {
    obj: ComplexObject,
}

impl UsableBuilder {
    fn create() -> UsableBuilder {
        UsableBuilder {
            obj: ComplexObject::create(),
        }
    }
}

impl Builder for UsableBuilder {
    fn push_part(&mut self, value: &str) {
        self.obj.parts.push(value.to_string());
    }

    fn get_build(&mut self) -> ComplexObject {
        let result = self.obj.clone();
        self.obj = ComplexObject::create();
        result
    }
}


pub fn use_case_one_part() -> Vec<String> {
    let mut builder = UsableBuilder::create();
    builder.push_part("parte a");
    let result = builder.get_build().parts;
    result
}

pub fn use_case_many_parts() -> Vec<String> {
    let mut builder = UsableBuilder::create();
    builder.push_part("parte a");
    builder.push_part("parte 2");
    builder.push_part("parte Z");
    let result = builder.get_build().parts;
    result
}