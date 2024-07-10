use radiate_extensions::operations::op;
use radiate_extensions::operations::op::Op;

fn main() {
    let add_op = op::add();

    let result = add_op.apply(&[1, 2]);

    println!("Result: {}", result);
}
