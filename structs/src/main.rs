#[derive(Debug)]

struct Variable {
    name: String,
    value: i32,
    address: u32,
}

impl Variable {
    fn inc(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut var: Variable = build_variable(String::from("var"), 10, 0);
    var.inc();
    //print_variable(var);

    let other_var = Variable {
        name: String::from("other"),
        ..var
    };

    print_variable(other_var);
}

fn print_variable(var: Variable) {
    println!("{:?}", var);

    let v = vec![1, 2, 3];
    let u = vec!['a'; 4];

    println!("{:?}", v);
    println!("{:?}", u);
}

fn build_variable(name: String, value: i32, address: u32) -> Variable {
    Variable {
        name,
        value,
        address,
    }
}
