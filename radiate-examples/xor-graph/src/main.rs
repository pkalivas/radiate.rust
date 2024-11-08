use radiate_extensions::operations::op;
use radiate_extensions::operations::op::Op;

fn main() {
    let add_op = op::add();
    let name = add_op.name();

    let result = add_op.apply(&[1, 2]);

    println!("{:?} Result: {}", add_op.name(), result);
}
