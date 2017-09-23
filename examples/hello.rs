extern crate nary;
use nary::Engine;

fn main()
{
	let mut engine = Engine::new();

	if let Ok(result) = engine.eval::<i64>("40 + 2")
	{
		println!("Answer: {}", result); // prints 42
	}
}
