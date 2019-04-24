use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::io;
use std::fs;

fn num_to_string (i:u32) -> String
{
	let mut vec : Vec<String> = vec!["0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string()];
	let hex = format!("{:x}", i);
	
	for x in 0 .. hex.len()
		{
			vec.pop();
		}
	vec.push(hex);
	let joined = vec.join("");
	return joined;
}
fn find_push(a: Vec<String>, v: Vec<&str>, flag: Vec<String>) -> String
{
	if v[1] == "true"
	{
		let op = "0002".to_string();
		return op;
	}
	else if v[1] == "false"
	
	{
		let op = "0003".to_string();
		return op;
	}
	
	else if v[1] == "undef"
	{
		let op = "0005".to_string();
		return op;
	}
	
	else{
	for x in 0 .. a.len()
	{	
		if v[1] == a[x]
		{
			let mut op = "0004".to_string();
			
			for y in 0 .. flag.len()
			{
				if flag[y] == a[x]
				{
					let num = num_to_string((x - y) as u32);
					op.push_str(&num);
					return op;
				}
			}
		}
	}	
	let mut op = "0001".to_string();
	let my_int = v[1].parse::<u32>().unwrap();
	let hold = num_to_string(my_int);
	
	op.push_str(&hold);
	return op;
	}
}

fn lines_from_file<P>(filename: P ) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
	
	let mut file = String::new();
	
    io::stdin().read_line(&mut file)
    	.ok()
        .expect("failed to read line");

	file.retain(|c| c != '\n');
	file.retain(|c| c != '\r');
	
	let mut o = file.clone();
	
    let lines = lines_from_file(file);
	let mut a = lines.clone();
	
	let mut c = 0;
	let mut flag : Vec<String> = vec!["0".to_string()];
	flag.pop();
	for x in 0 .. a.len()
	{
		if a[x].contains(":")
		{
			c = c + 1;
			a[x].retain(|c| c != ':');
			flag.push(a[x].to_string());
		}
	}

	let size : u32 = a.len() as u32;
	let mut s = num_to_string(size - c);
	for x in 0 .. a.len()
	{
		let v: Vec<&str> = a[x].split(' ').collect();
		for y in 0 .. 1
		{
			if v[y] == "push"
			{
				let b = a.clone();
				let c = v.clone();
				let d = flag.clone();
				s.push_str(&find_push(b,c,d));			
			}
			
			else if v[y] == "pop"
			{
				s.push_str(&"01".to_string());
			}
			
			else if v[y] == "peek"
			{
				let mut op = "02".to_string();
				let my_int = v[1].parse::<u32>().unwrap();
				op.push_str(&num_to_string(my_int));
				s.push_str(&op);
			}
			
			else if v[y] == "unary"
			{
				s.push_str(&"03".to_string());
				
				if v[y+1] == "neg"
				{
					s.push_str(&"00".to_string());
				}
			}
			
			else if v[y] == "binary"
			{
				s.push_str(&"04".to_string());
				
				if v[y+1] == "+"
				{
					s.push_str(&"00".to_string());
				}
				
				else if v[y+1] == "*"
				{
					s.push_str(&"01".to_string());
				}
				
				else if v[y+1] == "-"
				{
					s.push_str(&"02".to_string());
				}
				
				else if v[y+1] == "/"
				{
					s.push_str(&"03".to_string());
				}
				
				else if v[y+1] == "<" || v[y+1] == ">"
				{
					s.push_str(&"04".to_string());
				}
				
				else if v[y+1] == "=="
				{
					s.push_str(&"05".to_string());
				}
			}
			
			else if v[y] == "swap"
			{
				s.push_str(&"05".to_string());
			}
			
			else if v[y] == "alloc"
			{
				s.push_str(&"06".to_string());
			}
			
			else if v[y] == "set"
			{
				s.push_str(&"07".to_string());
			}
			
			else if v[y] == "get"
			{
				s.push_str(&"08".to_string());
			}
			
			else if v[y] == "var"
			{			
				let mut op = "09".to_string();
				let my_int = v[1].parse::<u32>().unwrap();
				op.push_str(&num_to_string(my_int));
				s.push_str(&op);
			}
			
			else if v[y] == "store"
			{			
				let mut op = "0a".to_string();
				let my_int = v[1].parse::<u32>().unwrap();
				op.push_str(&num_to_string(my_int));
				s.push_str(&op);
			}
			
			else if v[y] == "setframe"
			{	
				let mut op = "0b".to_string();
				let my_int = v[1].parse::<u32>().unwrap();
				op.push_str(&num_to_string(my_int));
				s.push_str(&op);
			}
			
			else if v[y] == "call"
			{
				s.push_str(&"0c".to_string());
			}
			
			else if v[y] == "ret"
			{
				s.push_str(&"0d".to_string());
			}
			
			else if v[y] == "branch"
			{
				s.push_str(&"0e".to_string());
			}
			
			else if v[y] == "halt"
			{
				s.push_str(&"0f".to_string());
			}
			else
			{
			}
		}
	}
	o.pop();
	o.push('o');
	let char_vec: Vec<char> = s.chars().collect();
	let mut vec: Vec<u8> = vec![];
	
	let mut i = 0;
	while i < char_vec.len()
	{
		if char_vec[i] == '0'
		{
			if char_vec[i+1] =='0'
			{vec.push(0);}
			else if char_vec[i+1] =='1'
			{vec.push(1);}
			else if char_vec[i+1] =='2'
			{vec.push(2);}
			else if char_vec[i+1] =='3'
			{vec.push(3);}
			else if char_vec[i+1] =='4'
			{vec.push(4);}
			else if char_vec[i+1] =='5'
			{vec.push(5);}
			else if char_vec[i+1] =='6'
			{vec.push(6);}
			else if char_vec[i+1] =='7'
			{vec.push(7);}
			else if char_vec[i+1] =='8'
			{vec.push(8);}
			else if char_vec[i+1] =='9'
			{vec.push(9);}
			else if char_vec[i+1] =='a'
			{vec.push(10);}
			else if char_vec[i+1] =='b'
			{vec.push(11);}
			else if char_vec[i+1] =='c'
			{vec.push(12);}
			else if char_vec[i+1] =='d'
			{vec.push(13);}
			else if char_vec[i+1] =='e'
			{vec.push(14);}
			else if char_vec[i+1] =='f'
			{vec.push(15);}
		}
		else if char_vec[i] == '1'
		{
			if char_vec[i+1] =='0'
			{vec.push(16);}
			else if char_vec[i+1] =='1'
			{vec.push(17);}
			else if char_vec[i+1] =='2'
			{vec.push(18);}
			else if char_vec[i+1] =='3'
			{vec.push(19);}
			else if char_vec[i+1] =='4'
			{vec.push(20);}
			else if char_vec[i+1] =='5'
			{vec.push(21);}
			else if char_vec[i+1] =='6'
			{vec.push(22);}
			else if char_vec[i+1] =='7'
			{vec.push(23);}
			else if char_vec[i+1] =='8'
			{vec.push(24);}
			else if char_vec[i+1] =='9'
			{vec.push(25);}
			else if char_vec[i+1] =='a'
			{vec.push(26);}
			else if char_vec[i+1] =='b'
			{vec.push(27);}
			else if char_vec[i+1] =='c'
			{vec.push(28);}
			else if char_vec[i+1] =='d'
			{vec.push(29);}
			else if char_vec[i+1] =='e'
			{vec.push(30);}
			else if char_vec[i+1] =='f'
			{vec.push(31);}
		}
		else if char_vec[i] == '2'
		{
			if char_vec[i+1] =='0'
			{vec.push(32);}
			else if char_vec[i+1] =='1'
			{vec.push(33);}
			else if char_vec[i+1] =='2'
			{vec.push(34);}
			else if char_vec[i+1] =='3'
			{vec.push(35);}
			else if char_vec[i+1] =='4'
			{vec.push(36);}
			else if char_vec[i+1] =='5'
			{vec.push(37);}
			else if char_vec[i+1] =='6'
			{vec.push(38);}
			else if char_vec[i+1] =='7'
			{vec.push(39);}
			else if char_vec[i+1] =='8'
			{vec.push(40);}
			else if char_vec[i+1] =='9'
			{vec.push(41);}
			else if char_vec[i+1] =='a'
			{vec.push(42);}
			else if char_vec[i+1] =='b'
			{vec.push(43);}
			else if char_vec[i+1] =='c'
			{vec.push(44);}
			else if char_vec[i+1] =='d'
			{vec.push(45);}
			else if char_vec[i+1] =='e'
			{vec.push(46);}
			else if char_vec[i+1] =='f'
			{vec.push(47);}
		}
		else if char_vec[i] == '3'
		{
			if char_vec[i+1] =='0'
			{vec.push(48);}
			else if char_vec[i+1] =='1'
			{vec.push(49);}
			else if char_vec[i+1] =='2'
			{vec.push(50);}
			else if char_vec[i+1] =='3'
			{vec.push(51);}
			else if char_vec[i+1] =='4'
			{vec.push(52);}
			else if char_vec[i+1] =='5'
			{vec.push(53);}
			else if char_vec[i+1] =='6'
			{vec.push(54);}
			else if char_vec[i+1] =='7'
			{vec.push(55);}
			else if char_vec[i+1] =='8'
			{vec.push(56);}
			else if char_vec[i+1] =='9'
			{vec.push(57);}
			else if char_vec[i+1] =='a'
			{vec.push(58);}
			else if char_vec[i+1] =='b'
			{vec.push(59);}
			else if char_vec[i+1] =='c'
			{vec.push(60);}
			else if char_vec[i+1] =='d'
			{vec.push(61);}
			else if char_vec[i+1] =='e'
			{vec.push(62);}
			else if char_vec[i+1] =='f'
			{vec.push(63);}
		}
		else if char_vec[i] == '4'
		{
			if char_vec[i+1] =='0'
			{vec.push(64);}
			else if char_vec[i+1] =='1'
			{vec.push(65);}
			else if char_vec[i+1] =='2'
			{vec.push(66);}
			else if char_vec[i+1] =='3'
			{vec.push(67);}
			else if char_vec[i+1] =='4'
			{vec.push(68);}
			else if char_vec[i+1] =='5'
			{vec.push(69);}
			else if char_vec[i+1] =='6'
			{vec.push(70);}
			else if char_vec[i+1] =='7'
			{vec.push(71);}
			else if char_vec[i+1] =='8'
			{vec.push(72);}
			else if char_vec[i+1] =='9'
			{vec.push(73);}
			else if char_vec[i+1] =='a'
			{vec.push(74);}
			else if char_vec[i+1] =='b'
			{vec.push(75);}
			else if char_vec[i+1] =='c'
			{vec.push(76);}
			else if char_vec[i+1] =='d'
			{vec.push(77);}
			else if char_vec[i+1] =='e'
			{vec.push(78);}
			else if char_vec[i+1] =='f'
			{vec.push(79);}
		}
		else if char_vec[i] == '5'
		{
			if char_vec[i+1] =='0'
			{vec.push(80);}
			else if char_vec[i+1] =='1'
			{vec.push(81);}
			else if char_vec[i+1] =='2'
			{vec.push(82);}
			else if char_vec[i+1] =='3'
			{vec.push(83);}
			else if char_vec[i+1] =='4'
			{vec.push(84);}
			else if char_vec[i+1] =='5'
			{vec.push(85);}
			else if char_vec[i+1] =='6'
			{vec.push(86);}
			else if char_vec[i+1] =='7'
			{vec.push(87);}
			else if char_vec[i+1] =='8'
			{vec.push(88);}
			else if char_vec[i+1] =='9'
			{vec.push(89);}
			else if char_vec[i+1] =='a'
			{vec.push(90);}
			else if char_vec[i+1] =='b'
			{vec.push(91);}
			else if char_vec[i+1] =='c'
			{vec.push(92);}
			else if char_vec[i+1] =='d'
			{vec.push(93);}
			else if char_vec[i+1] =='e'
			{vec.push(94);}
			else if char_vec[i+1] =='f'
			{vec.push(95);}
		}
		else if char_vec[i] == '6'
		{
			if char_vec[i+1] =='0'
			{vec.push(96);}
			else if char_vec[i+1] =='1'
			{vec.push(97);}
			else if char_vec[i+1] =='2'
			{vec.push(98);}
			else if char_vec[i+1] =='3'
			{vec.push(99);}
			else if char_vec[i+1] =='4'
			{vec.push(100);}
			else if char_vec[i+1] =='5'
			{vec.push(101);}
			else if char_vec[i+1] =='6'
			{vec.push(102);}
			else if char_vec[i+1] =='7'
			{vec.push(103);}
			else if char_vec[i+1] =='8'
			{vec.push(104);}
			else if char_vec[i+1] =='9'
			{vec.push(105);}
			else if char_vec[i+1] =='a'
			{vec.push(106);}
			else if char_vec[i+1] =='b'
			{vec.push(107);}
			else if char_vec[i+1] =='c'
			{vec.push(108);}
			else if char_vec[i+1] =='d'
			{vec.push(109);}
			else if char_vec[i+1] =='e'
			{vec.push(110);}
			else if char_vec[i+1] =='f'
			{vec.push(111);}
		}
		else if char_vec[i] == '7'
		{
			if char_vec[i+1] =='0'
			{vec.push(112);}
			else if char_vec[i+1] =='1'
			{vec.push(113);}
			else if char_vec[i+1] =='2'
			{vec.push(114);}
			else if char_vec[i+1] =='3'
			{vec.push(115);}
			else if char_vec[i+1] =='4'
			{vec.push(116);}
			else if char_vec[i+1] =='5'
			{vec.push(117);}
			else if char_vec[i+1] =='6'
			{vec.push(118);}
			else if char_vec[i+1] =='7'
			{vec.push(119);}
			else if char_vec[i+1] =='8'
			{vec.push(120);}
			else if char_vec[i+1] =='9'
			{vec.push(121);}
			else if char_vec[i+1] =='a'
			{vec.push(122);}
			else if char_vec[i+1] =='b'
			{vec.push(123);}
			else if char_vec[i+1] =='c'
			{vec.push(124);}
			else if char_vec[i+1] =='d'
			{vec.push(125);}
			else if char_vec[i+1] =='e'
			{vec.push(126);}
			else if char_vec[i+1] =='f'
			{vec.push(127);}
		}
		else if char_vec[i] == '8'
		{
			if char_vec[i+1] =='0'
			{vec.push(128);}
			else if char_vec[i+1] =='1'
			{vec.push(129);}
			else if char_vec[i+1] =='2'
			{vec.push(130);}
			else if char_vec[i+1] =='3'
			{vec.push(131);}
			else if char_vec[i+1] =='4'
			{vec.push(132);}
			else if char_vec[i+1] =='5'
			{vec.push(133);}
			else if char_vec[i+1] =='6'
			{vec.push(134);}
			else if char_vec[i+1] =='7'
			{vec.push(135);}
			else if char_vec[i+1] =='8'
			{vec.push(136);}
			else if char_vec[i+1] =='9'
			{vec.push(137);}
			else if char_vec[i+1] =='a'
			{vec.push(138);}
			else if char_vec[i+1] =='b'
			{vec.push(139);}
			else if char_vec[i+1] =='c'
			{vec.push(140);}
			else if char_vec[i+1] =='d'
			{vec.push(141);}
			else if char_vec[i+1] =='e'
			{vec.push(142);}
			else if char_vec[i+1] =='f'
			{vec.push(143);}
		}
		else if char_vec[i] == '9'
		{
			if char_vec[i+1] =='0'
			{vec.push(144);}
			else if char_vec[i+1] =='1'
			{vec.push(145);}
			else if char_vec[i+1] =='2'
			{vec.push(146);}
			else if char_vec[i+1] =='3'
			{vec.push(147);}
			else if char_vec[i+1] =='4'
			{vec.push(148);}
			else if char_vec[i+1] =='5'
			{vec.push(149);}
			else if char_vec[i+1] =='6'
			{vec.push(150);}
			else if char_vec[i+1] =='7'
			{vec.push(151);}
			else if char_vec[i+1] =='8'
			{vec.push(152);}
			else if char_vec[i+1] =='9'
			{vec.push(153);}
			else if char_vec[i+1] =='a'
			{vec.push(154);}
			else if char_vec[i+1] =='b'
			{vec.push(155);}
			else if char_vec[i+1] =='c'
			{vec.push(156);}
			else if char_vec[i+1] =='d'
			{vec.push(157);}
			else if char_vec[i+1] =='e'
			{vec.push(158);}
			else if char_vec[i+1] =='f'
			{vec.push(159);}
		}
		else if char_vec[i] == 'a'
		{
			if char_vec[i+1] =='0'
			{vec.push(160);}
			else if char_vec[i+1] =='1'
			{vec.push(161);}
			else if char_vec[i+1] =='2'
			{vec.push(162);}
			else if char_vec[i+1] =='3'
			{vec.push(163);}
			else if char_vec[i+1] =='4'
			{vec.push(164);}
			else if char_vec[i+1] =='5'
			{vec.push(165);}
			else if char_vec[i+1] =='6'
			{vec.push(166);}
			else if char_vec[i+1] =='7'
			{vec.push(167);}
			else if char_vec[i+1] =='8'
			{vec.push(168);}
			else if char_vec[i+1] =='9'
			{vec.push(169);}
			else if char_vec[i+1] =='a'
			{vec.push(170);}
			else if char_vec[i+1] =='b'
			{vec.push(171);}
			else if char_vec[i+1] =='c'
			{vec.push(172);}
			else if char_vec[i+1] =='d'
			{vec.push(173);}
			else if char_vec[i+1] =='e'
			{vec.push(174);}
			else if char_vec[i+1] =='f'
			{vec.push(175);}
		}
		else if char_vec[i] == 'b'
		{
			if char_vec[i+1] =='0'
			{vec.push(176);}
			else if char_vec[i+1] =='1'
			{vec.push(177);}
			else if char_vec[i+1] =='2'
			{vec.push(178);}
			else if char_vec[i+1] =='3'
			{vec.push(179);}
			else if char_vec[i+1] =='4'
			{vec.push(180);}
			else if char_vec[i+1] =='5'
			{vec.push(181);}
			else if char_vec[i+1] =='6'
			{vec.push(182);}
			else if char_vec[i+1] =='7'
			{vec.push(183);}
			else if char_vec[i+1] =='8'
			{vec.push(184);}
			else if char_vec[i+1] =='9'
			{vec.push(185);}
			else if char_vec[i+1] =='a'
			{vec.push(186);}
			else if char_vec[i+1] =='b'
			{vec.push(187);}
			else if char_vec[i+1] =='c'
			{vec.push(188);}
			else if char_vec[i+1] =='d'
			{vec.push(189);}
			else if char_vec[i+1] =='e'
			{vec.push(190);}
			else if char_vec[i+1] =='f'
			{vec.push(191);}
		}
		else if char_vec[i] == 'c'
		{
			if char_vec[i+1] =='0'
			{vec.push(192);}
			else if char_vec[i+1] =='1'
			{vec.push(193);}
			else if char_vec[i+1] =='2'
			{vec.push(194);}
			else if char_vec[i+1] =='3'
			{vec.push(195);}
			else if char_vec[i+1] =='4'
			{vec.push(196);}
			else if char_vec[i+1] =='5'
			{vec.push(197);}
			else if char_vec[i+1] =='6'
			{vec.push(198);}
			else if char_vec[i+1] =='7'
			{vec.push(199);}
			else if char_vec[i+1] =='8'
			{vec.push(200);}
			else if char_vec[i+1] =='9'
			{vec.push(201);}
			else if char_vec[i+1] =='a'
			{vec.push(202);}
			else if char_vec[i+1] =='b'
			{vec.push(203);}
			else if char_vec[i+1] =='c'
			{vec.push(204);}
			else if char_vec[i+1] =='d'
			{vec.push(205);}
			else if char_vec[i+1] =='e'
			{vec.push(206);}
			else if char_vec[i+1] =='f'
			{vec.push(207);}
		}
		else if char_vec[i] == 'd'
		{
			if char_vec[i+1] =='0'
			{vec.push(208);}
			else if char_vec[i+1] =='1'
			{vec.push(209);}
			else if char_vec[i+1] =='2'
			{vec.push(210);}
			else if char_vec[i+1] =='3'
			{vec.push(211);}
			else if char_vec[i+1] =='4'
			{vec.push(212);}
			else if char_vec[i+1] =='5'
			{vec.push(213);}
			else if char_vec[i+1] =='6'
			{vec.push(214);}
			else if char_vec[i+1] =='7'
			{vec.push(215);}
			else if char_vec[i+1] =='8'
			{vec.push(216);}
			else if char_vec[i+1] =='9'
			{vec.push(217);}
			else if char_vec[i+1] =='a'
			{vec.push(218);}
			else if char_vec[i+1] =='b'
			{vec.push(219);}
			else if char_vec[i+1] =='c'
			{vec.push(220);}
			else if char_vec[i+1] =='d'
			{vec.push(221);}
			else if char_vec[i+1] =='e'
			{vec.push(222);}
			else if char_vec[i+1] =='f'
			{vec.push(223);}
		}
		else if char_vec[i] == 'e'
		{
			if char_vec[i+1] =='0'
			{vec.push(224);}
			else if char_vec[i+1] =='1'
			{vec.push(225);}
			else if char_vec[i+1] =='2'
			{vec.push(226);}
			else if char_vec[i+1] =='3'
			{vec.push(227);}
			else if char_vec[i+1] =='4'
			{vec.push(228);}
			else if char_vec[i+1] =='5'
			{vec.push(229);}
			else if char_vec[i+1] =='6'
			{vec.push(230);}
			else if char_vec[i+1] =='7'
			{vec.push(231);}
			else if char_vec[i+1] =='8'
			{vec.push(232);}
			else if char_vec[i+1] =='9'
			{vec.push(233);}
			else if char_vec[i+1] =='a'
			{vec.push(234);}
			else if char_vec[i+1] =='b'
			{vec.push(235);}
			else if char_vec[i+1] =='c'
			{vec.push(236);}
			else if char_vec[i+1] =='d'
			{vec.push(237);}
			else if char_vec[i+1] =='e'
			{vec.push(238);}
			else if char_vec[i+1] =='f'
			{vec.push(239);}
		}
		else if char_vec[i] == 'f'
		{
			if char_vec[i+1] =='0'
			{vec.push(240);}
			else if char_vec[i+1] =='1'
			{vec.push(241);}
			else if char_vec[i+1] =='2'
			{vec.push(242);}
			else if char_vec[i+1] =='3'
			{vec.push(243);}
			else if char_vec[i+1] =='4'
			{vec.push(244);}
			else if char_vec[i+1] =='5'
			{vec.push(245);}
			else if char_vec[i+1] =='6'
			{vec.push(246);}
			else if char_vec[i+1] =='7'
			{vec.push(247);}
			else if char_vec[i+1] =='8'
			{vec.push(248);}
			else if char_vec[i+1] =='9'
			{vec.push(249);}
			else if char_vec[i+1] =='a'
			{vec.push(250);}
			else if char_vec[i+1] =='b'
			{vec.push(251);}
			else if char_vec[i+1] =='c'
			{vec.push(252);}
			else if char_vec[i+1] =='d'
			{vec.push(253);}
			else if char_vec[i+1] =='e'
			{vec.push(254);}
			else if char_vec[i+1] =='f'
			{vec.push(255);}
		}
		i = i + 2;
	}
	
	fs::write(o, vec).expect("Could not write to file");
}