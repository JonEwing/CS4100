//Hw0 Part2
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
	let mut nums = vec![];
	let mut aux = vec![];
	let mut hold: usize = 0;
	let mut hold_aux: usize = 0;
	let mut big = 0;
	let mut big_aux = 0;
	
	for x in 0..v.len() 
		{				
			if v[x] == "+"
			{
				if nums.len() < 2
				{
					print!("-1");
					panic!("Not Enough Numbers");
				}
				
				nums.pop();
				nums.pop();
				nums.push(1);
			}
			else if v[x] == "-"
			{
				if nums.len() < 2
				{
					print!("-1");
					panic!("Not Enough Numbers");
				}
				
				nums.pop();
				nums.pop();
				nums.push(1);
			}
		
			else if v[x] == "*"
			{
				if nums.len() < 2
				{
					print!("-1");
					panic!("Not Enough Numbers");
				}
				
				nums.pop();
				nums.pop();
				nums.push(1);
			}
	
			else if v[x] == "/"
			{
				if nums.len() < 2
				{
					print!("-1");
					panic!("Not Enough Numbers");
				}
				
				nums.pop();
				nums.pop();
				nums.push(1);
			}
			
			else if v[x] == "save"
			{	
				if nums.len() != 1
				{
					print!("-1");
					panic!("Not Enough Numbers");
				}
				
				aux.push(1);
				nums.pop();
			}
			
			else if v[x] == "restore"
			{
				if aux.len() != 1
				{
					print!("-1");
					panic!("No Save Avalable");
				}
				aux.pop();
				nums.push(1);
			}
			
			else if v[x] == "done"
			{
				if nums.len() != 1
				{
					print!("-1");
					panic!("Not Enough Numbers");
				}
				print!("{}",big + big_aux);
			}
			
			else
			{
				nums.push(1);
			}
			
			if hold < nums.len()
			{
				hold = nums.len();
                big = hold as i32;
			}
			
			if hold_aux < aux.len()
			{
				hold_aux = aux.len();
                big_aux = hold_aux as i32;
			}
		}
	if v[v.len()-1] != "done"
	{
		print!("-1");
		panic!("'done' not included");
	}
}