
mod stack;

fn main() {

    let mut stack = stack::Stack::<u32>::with_capacity(10);
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    stack.push(6);
    stack.push(7);
    stack.push(8);
    stack.push(9);
    stack.push(10000);
    stack.push(11);
    stack.push(12);
    stack.push(13);
    let popped = stack.pop();
        match popped {
            Some(v) => println!("{}", v),
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

