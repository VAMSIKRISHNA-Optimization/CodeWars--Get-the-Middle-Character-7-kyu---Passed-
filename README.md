# CodeWars--Get-the-Middle-Character-7-kyu---Passed-
You are going to be given a non-empty string. Your job is to return the middle character(s) of the string.

If the string's length is odd, return the middle character.
If the string's length is even, return the middle 2 characters.
Examples:
"test" --> "es"
"testing" --> "t"
"middle" --> "dd"
"A" --> "A"



TEST CASES:
#[test]
fn basic_tests() {
  assert_eq!(get_middle("test"),"es");
  assert_eq!(get_middle("testing"),"t");
  assert_eq!(get_middle("middle"),"dd");
  assert_eq!(get_middle("another"),"t");
  assert_eq!(get_middle("123456"),"34");
  assert_eq!(get_middle("A"),"A");
  assert_eq!(get_middle("of"),"of");
}
