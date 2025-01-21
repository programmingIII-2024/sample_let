fn main()
{
	let n = 0;
	let mut m =0;

//	m =5;	// 使用前にmutableな変数が書き換わると警告が出る


	println!("n: {}", n);
	println!("m: {}", m);

	m=1;
//	n=1;		// コメントアウトを外してimmutableな変数に代入してみる
	println!("n: {}", n);
	println!("m: {}", m);
}
