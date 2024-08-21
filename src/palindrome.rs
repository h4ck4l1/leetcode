#![allow(unused)]

/*

def is_palindrome(x:int) -> bool:
    a = str(x)
    for i in range(len(a)//2):
        if a[i:i+1] != a[len(a)-i-1:len(a)-i]:
            return False
    
    return True
        
is_palindrome(121)

*/
pub fn is_palindrome(x:i32) -> bool {
    if x < 0 {
        return false;
    } else if (x >= 0) & (x < 10) {
        return true;
    }
    let mut number = x; // 42324
    let mut holding_number = 0;
    loop {
        if number > 0 {
            let moving_number = number%10; // moving = 4, 2, 3
            holding_number  = (holding_number * 10) + moving_number; // holding = 40, 420, 4230
            number = (number - moving_number)/10; // number = 42324 - 4 * 10, 4232
        } else {
            break;
        }
    }
    holding_number == x
}