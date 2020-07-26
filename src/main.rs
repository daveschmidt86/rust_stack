
mod stack;


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let mut stack = stack::Stack::<User>::with_capacity(10);

    for n in 0..11 {
        let some_user = User {
            username: String::from("tester"),
            email: String::from("something@something.com"),
            sign_in_count: n,
            active: true
        };
        stack.push(some_user);
    }
    let popped = stack.pop();
        match popped {
            Some(v) => 
            {
                println!("{}", v.username);
                println!("{}", v.sign_in_count);
                println!("{}", v.email);
            },
            None => println!("stack full")
        }
    println!("{}", stack.size());
}


fn make_a_test_add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn does_adding_work() {
        assert_eq!(make_a_test_add(2,3), 5);
    }
}

