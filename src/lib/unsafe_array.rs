
const WHITESPACE: char = '\u{0020}';

pub fn run_unsafe_array() {
   let mut arr: Vec<char> = (0..18)
       .into_iter()
       .map(|i| match i {
           0 => '[',
           17 => ']',
           _ => WHITESPACE,
       })
       .collect();

   let mut range = (0..18).chain((1..17).rev()).cycle();
   let mut prev = 0;
   let not_border = |prev: usize| prev != 0 && prev != 17;

   loop {
       unsafe {
           let i = range.next().unwrap();
           let char = arr.get_unchecked_mut(i);

           match &char {
               '\u{0020}' => {
                   // 1..=16
                   *char = '#';

                   if prev != i && not_border(prev) {
                       *arr.get_unchecked_mut(prev) = WHITESPACE;
                   }

                   println!("{:?}", arr);
                   std::thread::sleep(std::time::Duration::from_millis(200));
               }
               _ => (),
           }
           prev = i;
       }
   }
}
