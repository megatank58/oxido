use crate::datatype::Data;

pub struct Globals;

impl Globals {
    pub fn _has(x: &str) -> bool {
        ["print", "println"].contains(&x)
    }

    pub fn call(x: &str, params: Vec<Data>) {
        match x {
            "print" => Globals::print(params),
            "println" => Globals::println(params),
            _ => panic!("not a global function")
        }
    }

    pub fn print(data: Vec<Data>) {
        print::print(data);
    }

    pub fn println(data: Vec<Data>) {
        Globals::print(data);
        Globals::print(vec![Data::String("\n".to_string())])
    }
}

mod print {
    use crate::datatype::Data;

    pub fn print(datas: Vec<Data>) {
        for data in datas {
            match data {
                Data::Integer(i) => print!("{i}"),
                Data::Bool(b) => print!("{b}"),
                Data::String(s) => print!("{s}"),
            }
        }
    }
}
