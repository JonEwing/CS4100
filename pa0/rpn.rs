//Hw0 Part1
//CS4100
//Jonathan Feige

use std::io;

fn main() {
	
	let mut cal = String::new();
	
    io::stdin().read_line(&mut cal)
    	.ok()
        .expect("failed to read line");

	cal.retain(|c| c != '\n');
	cal.retain(|c| c != '\r');
	
	let v: Vec<&str> = cal.split(' ').collect();
	let mut nums: Vec<i32> = vec![];
	let mut aux = vec![];
	
	for x in 0..v.len() 
		{				
			if v[x] == "+"
			{	
				if nums.len() < 2
				{
					panic!("Not Enough Numbers");
				}
							
	    		let a = nums[nums.len()-1];
				let b = nums[nums.len()-2];
				
				nums.pop();
				nums.pop();
				nums.push(b+a);
			}
		
			else if v[x] == "-"
			{
				if nums.len() < 2
				{
					panic!("Not Enough Numbers");
				}
				
    			let a = nums[nums.len()-1];
				let b = nums[nums.len()-2];
				
				nums.pop();
				nums.pop();
				nums.push(b-a);
			}
		
			else if v[x] == "*"
			{
				if nums.len() < 2
				{
					panic!("Not Enough Numbers");
				}
				
    			let a = nums[nums.len()-1];
				let b = nums[nums.len()-2];
				
				nums.pop();
				nums.pop();
				nums.push(b*a);
			}
	
			else if v[x] == "/"
			{
				if nums.len() < 2
				{
					panic!("Not Enough Numbers");
				}
				
    			let a = nums[nums.len()-1];
				let b = nums[nums.len()-2];
				
				if a == 0
				{
					panic!("Divide by Zero");
				}
				
				nums.pop();
				nums.pop();
				nums.push(b/a);
			}
			
			else if v[x] == "save"
			{	
				if nums.len() != 1
				{
					panic!("Not Enough Numbers");
				}
				
				let a  = nums[nums.len()-1];
				
				aux.push(a);
				nums.pop();
			}
			
			else if v[x] == "restore"
			{
				if nums.len() != 1
				{
					panic!("Not Enough Numbers");
				}
				
				let a  = aux[0];
				aux.pop();
				nums.push(a);
			}
			
			else if v[x] == "done"
			{
				if nums.len() != 1
				{
					panic!("Not Enough Numbers");
				}
				
    			print!("{}",nums[nums.len()-1]);
			}
			
			else
			{
				let my_int = v[x].parse::<i32>().unwrap();
				nums.push(my_int);
			}
		}
	if v[v.len()-1] != "done"
		{
			panic!("'done' not included");
		}
}
