pub fn my_atoi(s: String) -> i32 {
    
    /* let mut sign_check:bool = false;
    let mut neg_number: bool=false;
    let mut answer: i32=0; */

    fn remove_leading_spaces_and_more(s: String)->String{
        let mut answer = "".to_string();
        let mut first_non_empty_char: bool=false;
        
        for c in s.chars() {
            
            if c==' ' && first_non_empty_char==false{
                continue;
            }
            if c==' ' && first_non_empty_char==true{
                return answer;
            }
            if c!=' ' && first_non_empty_char==false{
                first_non_empty_char=true;
            }
            if c!=' ' && first_non_empty_char==true{
                answer.push(c);
            }
        }
        return answer
    }

    fn remove_signs(s: String)->(String, bool){
        let mut answer = "".to_string();
        let mut first_sign_handled: bool=false;
        let mut sign_is_negative: bool = false;
        
        for c in s.chars() {
            
            if c=='-' && first_sign_handled==false{
                first_sign_handled=true;
                sign_is_negative=true;
                continue;
            }
            if c=='+' && first_sign_handled==false{
                first_sign_handled=true;
                continue;
            }
            if (c!='-' && c!='+') && first_sign_handled==false {
                first_sign_handled=true;
            }
            answer.push(c);
        }
        return (answer, sign_is_negative);
    }

    fn remove_leading_zeros(s: String) -> String {
        let mut answer = "".to_string();
        let mut has_leading_zeros:bool=true;

        for c in s.chars() {
            
            if has_leading_zeros==true && c=='0'{
                continue;
            }
            if has_leading_zeros==true && c!='0'{
                has_leading_zeros=false;
            }
            if has_leading_zeros==false{
                answer.push(c);
            }
        }

        return answer
    }

    fn get_numbers_in_string(s: String)->String{
        let mut answer="".to_string();

        for c in s.chars() {
            if  c=='1' ||
                c=='2' ||
                c=='3' ||
                c=='4' ||
                c=='5' ||
                c=='6' ||
                c=='7' ||
                c=='8' ||
                c=='9' ||
                c=='0' {
                    answer.push(c);
                } else {
                    return answer;
                }

        }

        return answer;
    }

    let s: String=remove_leading_spaces_and_more(s);
    let (s, sign_is_negative)=remove_signs(s);
    let s: String=remove_leading_zeros(s);
    let mut s: String=get_numbers_in_string(s);
    if s==""{
        s="0".to_string();
    }

    let mut answer = match s.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            return if sign_is_negative { -2147483648 } else { 2147483647 };
        }
    };

    if sign_is_negative {
        answer = -answer;
    }

  
    return answer as i32
}