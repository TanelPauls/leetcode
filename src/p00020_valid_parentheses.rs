pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => return false,
        }
    }

    stack.is_empty()
}

/* pub fn is_valid(mut s: String)->bool {
    if s.len()%2!=0 {
        return false;
    }
    let mut pointer = 0usize;

    //println!("{}", &s);
    /* println!("{}", &s[0..1]);

    
    s.remove(1);
    s.remove(0);
    println!("{}", &s[0..1]);

    println!("{}", &s); */
    //println!("{}", s.len());

    //println!("{}", &s[pointer_1..pointer_2]);
    //while s.len()>=2  {
/*     if (&s[pointer_1..pointer_1+1]=="(" && &s[pointer_2..pointer_2+1]==")") {
        println!("yes");
    } */
    while pointer<s.len()/2 {
        //println!("asd: {}", &s);
        if (&s[pointer..pointer+1]=="(" && &s[pointer+1..pointer+2]==")") {
            s.remove(pointer);
            s.remove(pointer);
            if pointer>=1{
                pointer-=1;
            }
            continue;
        }
        if (&s[pointer..pointer+1]=="{" && &s[pointer+1..pointer+2]=="}") {
            s.remove(pointer);
            s.remove(pointer);
            if pointer>=1{
                pointer-=1;
            }
            continue;
        }
        if (&s[pointer..pointer+1]=="[" && &s[pointer+1..pointer+2]=="]") {
            s.remove(pointer);
            s.remove(pointer);
            if pointer>=1{
                pointer-=1;
            }
            continue;
        }
        pointer+=1;
    }
    if s.len()==0{
        return true;
    }
    //println!("{}", &s);
    false
} */